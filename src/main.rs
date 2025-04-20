// src/main.rs
// Entry point for Irontide (Rust port of Newsboat)
extern crate clap;
extern crate crossterm;
extern crate gettext;
extern crate libc;
extern crate openssl;
extern crate ratatui;
extern crate rss;

mod cache;
mod config;
mod configpaths;
mod controller;
mod dbexception;
mod exception;
mod matcherexception;
mod parser;
mod stflpp;
mod utils;
mod view;
mod xlicense;

use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use libc::{uname, utsname};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::{env, path::PathBuf, process};

// CLI arguments struct, derived from C++ options in newsboat.cpp
#[derive(Debug, Parser)]
#[clap(
    name = "irontide",
    version = utils::program_version(),
    about = "Rust port of Newsboat"
)]
pub struct CliArgs {
    #[clap(
        short = 'e',
        long = "export-to-opml",
        help = "export OPML feed to stdout"
    )]
    pub export_to_opml: bool,

    #[clap(
        long = "export-to-opml2",
        help = "export OPML 2.0 feed including tags to stdout"
    )]
    pub export_to_opml2: bool,

    #[clap(
        short = 'r',
        long = "refresh-on-start",
        help = "refresh feeds on start"
    )]
    pub refresh_on_start: bool,

    #[clap(
        short = 'i',
        long = "import-from-opml",
        value_name = "file",
        help = "import OPML file"
    )]
    pub import_from_opml: Option<PathBuf>,

    #[clap(
        short = 'u',
        long = "url-file",
        value_name = "file",
        help = "read RSS feed URLs from file"
    )]
    pub url_file: Option<PathBuf>,

    #[clap(
        short = 'c',
        long = "cache-file",
        value_name = "file",
        help = "use file as cache file"
    )]
    pub cache_file: Option<PathBuf>,

    #[clap(
        short = 'C',
        long = "config-file",
        value_name = "file",
        help = "read configuration from file"
    )]
    pub config_file: Option<PathBuf>,

    #[clap(
        long = "queue-file",
        value_name = "file",
        help = "use file as podcast queue file"
    )]
    pub queue_file: Option<PathBuf>,

    #[clap(
        long = "search-history-file",
        value_name = "file",
        help = "save search history to file"
    )]
    pub search_history_file: Option<PathBuf>,

    #[clap(
        long = "cmdline-history-file",
        value_name = "file",
        help = "save command-line history to file"
    )]
    pub cmdline_history_file: Option<PathBuf>,

    #[clap(short = 'X', long = "vacuum", help = "compact the cache")]
    pub vacuum: bool,

    #[clap(
        short = 'x',
        long = "execute",
        value_name = "command",
        multiple_occurrences = true,
        help = "execute list of commands"
    )]
    pub execute: Vec<String>,

    #[clap(short = 'q', long = "quiet", help = "quiet startup")]
    pub quiet: bool,

    #[clap(short = 'v', long = "version", help = "print version information")]
    pub version: bool,

    #[clap(
        short = 'l',
        long = "log-level",
        value_name = "loglevel",
        help = "write a log with given log level (1-6)"
    )]
    pub log_level: Option<u8>,

    #[clap(
        short = 'd',
        long = "log-file",
        value_name = "file",
        help = "use file as output log file"
    )]
    pub log_file: Option<PathBuf>,

    #[clap(
        short = 'E',
        long = "export-to-file",
        value_name = "file",
        help = "export list of read articles to file"
    )]
    pub export_to_file: Option<PathBuf>,

    #[clap(
        short = 'I',
        long = "import-from-file",
        value_name = "file",
        help = "import list of read articles from file"
    )]
    pub import_from_file: Option<PathBuf>,

    #[clap(short = 'h', long = "help", help = "display help message")]
    pub help: bool,

    #[clap(long = "cleanup", help = "remove unreferenced items from cache")]
    pub cleanup: bool,
}

extern "C" {
    fn rs_setup_human_panic();
}

fn print_usage(argv0: &str, configpaths: &configpaths::ConfigPaths) {
    let msg = format!(
        "{} {}\nusage: {} [OPTIONS]\n",
        xlicense::PROGRAM_NAME,
        utils::program_version(),
        argv0
    );
    println!("{}", msg);
}

fn print_version(argv0: &str, level: u32) {
    if level <= 1 {
        println!(
            "{} {} - {}",
            xlicense::PROGRAM_NAME,
            utils::program_version(),
            xlicense::PROGRAM_URL
        );
        let mut uts: utsname = unsafe { std::mem::zeroed() };
        unsafe { uname(&mut uts) };
    } else {
        println!("{}", xlicense::LICENSE_STR);
    }
}

fn main() {
    // Panic hook
    unsafe { rs_setup_human_panic() };
    // SSL
    utils::initialize_ssl_implementation();
    // Localization
    gettext::bind_textdomain_codeset(xlicense::PACKAGE, "UTF-8");
    gettext::textdomain(xlicense::PACKAGE);
    // RSS parser init
    parser::Parser::global_init();
    // Config paths
    let configpaths = configpaths::ConfigPaths::new();
    if !configpaths.initialized() {
        eprintln!("{}", configpaths.error_message());
        process::exit(1);
    }
    // Terminal
    enable_raw_mode().expect("raw mode");
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    // Parse args
    let args = CliArgs::parse();
    configpaths.process_args(&args);
    // Help/version
    if args.help {
        print_usage(&args.program_name(), &configpaths);
        return;
    } else if args.version {
        print_version(&args.program_name(), 1);
        return;
    }
    // Controller
    let mut controller = controller::Controller::new(&configpaths);
    let view = view::View::new(&controller);
    controller.set_view(&view);
    let exit_code = match controller.run(&args) {
        Ok(code) => code,
        Err(e) => {
            stflpp::Stfl::reset();
            eprintln!("Error: {}", e);
            1
        }
    };
    // Cleanup
    parser::Parser::global_cleanup();
    execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
    process::exit(exit_code);
}
