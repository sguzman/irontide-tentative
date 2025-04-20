extern crate clap;
extern crate rusqlite;
extern crate reqwest;
extern crate log;

use clap::{App, Arg};
use rusqlite::Connection;
use reqwest::Client;
use std::fs::File;
use std::io::{self, Read};

const PROGRAM_NAME: &str = "irontide";
const PACKAGE: &str = "irontide";
const LOCALEDIR: &str = "/usr/share/locale";

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

fn main() {
    let matches = App::new(PROGRAM_NAME)
        .version(utils::program_version())
        .author("Alexander Batischev")
        .about("A simple RSS reader")
        .arg(
            Arg::with_name("import-from-opml")
                .short('i')
                .long("import-from-opml")
                .value_name("FILE")
                .help("Import OPML file"),
        )
        .arg(
            Arg::with_name("url-file")
                .short('u')
                .long("url-file")
                .value_name("FILE")
                .help("Read RSS feed URLs from FILE"),
        )
        .arg(
            Arg::with_name("cache-file")
                .short('c')
                .long("cache-file")
                .value_name("FILE")
                .help("Use FILE as cache file"),
        )
        .arg(
            Arg::with_name("config-file")
                .short('C')
                .long("config-file")
                .value_name("FILE")
                .help("Read configuration from FILE"),
        )
        .arg(
            Arg::with_name("queue-file")
                .long("queue-file")
                .value_name("FILE")
                .help("Use FILE as podcast queue file"),
        )
        .arg(
            Arg::with_name("search-history-file")
                .long("search-history-file")
                .value_name("FILE")
                .help("Save the input history of the search to FILE"),
        )
        .arg(
            Arg::with_name("cmdline-history-file")
                .long("cmdline-history-file")
                .value_name("FILE")
                .help("Save the input history of the command line to FILE"),
        )
        .arg(
            Arg::with_name("vacuum")
                .short('X')
                .long("vacuum")
                .help("Compact the cache"),
        )
        .arg(
            Arg::with_name("execute")
                .short('x')
                .long("execute")
                .value_name("COMMANDS...")
                .multiple(true)
                .help("Execute list of commands"),
        )
        .arg(
            Arg::with_name("quiet")
                .short('q')
                .long("quiet")
                .help("Quiet startup"),
        )
        .arg(
            Arg::with_name("version")
                .short('v')
                .long("version")
                .help("Get version information"),
        )
        .arg(
            Arg::with_name("log-level")
                .short('l')
                .long("log-level")
                .value_name("LOGLEVEL")
                .takes_value(true)
                .help("Write a log with a certain log level (1 to 6 for user error, critical, error, warning, info, and debug respectively)"),
        )
        .arg(
            Arg::with_name("log-file")
                .short('d')
                .long("log-file")
                .value_name("FILE")
                .help("Use FILE as output log file"),
        )
        .arg(
            Arg::with_name("export-to-file")
                .short('E')
                .long("export-to-file")
                .value_name("FILE")
                .help("Export list of read articles to FILE"),
        )
        .arg(
            Arg::with_name("import-from-file")
                .short('I')
                .long("import-from-file")
                .value_name("FILE")
                .help("Import list of read articles from FILE"),
        )
        .arg(
            Arg::with_name("help")
                .short('h')
                .long("help")
                .help("This help"),
        )
        .arg(
            Arg::with_name("cleanup")
                .long("cleanup")
                .help("Remove unreferenced items from cache"),
        )
        .get_matches();

    let configpaths = ConfigPaths::new();
    if !configpaths.initialized() {
        eprintln!("Error: {}", configpaths.error_message());
        std::process::exit(EXIT_FAILURE);
    }

    let mut c = Controller::new(configpaths);
    let mut v = View::new(&c);
    c.set_view(&v);

    if matches.is_present("import-from-opml") {
        // Handle import-from-opml
    }
    if matches.is_present("url-file") {
        // Handle url-file
    }
    if matches.is_present("cache-file") {
        // Handle cache-file
    }
    if matches.is_present("config-file") {
        // Handle config-file
    }
    if matches.is_present("queue-file") {
        // Handle queue-file
    }
    if matches.is_present("search-history-file") {
        // Handle search-history-file
    }
    if matches.is_present("cmdline-history-file") {
        // Handle cmdline-history-file
    }
    if matches.is_present("vacuum") {
        // Handle vacuum
    }
    if let Some(commands) = matches.values_of("execute") {
        for command in commands {
            // Handle execute command
        }
    }
    if matches.is_present("quiet") {
        // Handle quiet
    }
    if matches.is_present("version") {
        print_version(PROGRAM_NAME, 1);
        std::process::exit(EXIT_SUCCESS);
    }
    if let Some(log_level) = matches.value_of("log-level") {
        // Handle log-level
    }
    if let Some(log_file) = matches.value_of("log-file") {
        // Handle log-file
    }
    if let Some(export_to_file) = matches.value_of("export-to-file") {
        // Handle export-to-file
    }
    if let Some(import_from_file) = matches.value_of("import-from-file") {
        // Handle import-from-file
    }
    if matches.is_present("help") {
        print_usage(PROGRAM_NAME, configpaths);
        std::process::exit(EXIT_SUCCESS);
    }
    if matches.is_present("cleanup") {
        // Handle cleanup
    }

    let ret = c.run(matches);
    rsspp::Parser::global_cleanup();
    std::process::exit(ret);
}

// Dummy implementations for missing functions and structs
struct ConfigPaths;
impl ConfigPaths {
    fn new() -> Self {
        ConfigPaths
    }
    fn initialized(&self) -> bool {
        true
    }
    fn error_message(&self) -> String {
        String::new()
    }
    fn process_args(&self, _args: CliArgsParser) {}
}

struct Controller;
impl Controller {
    fn new(_configpaths: ConfigPaths) -> Self {
        Controller
    }
    fn set_view(&mut self, _v: &View) {}
    fn run(&self, _args: CliArgsParser) -> i32 {
        0
    }
}

struct View;
impl View;

// Dummy implementation for missing structs and functions
struct CliArgsParser;
struct Stfl;
fn strprintf::fmt(_format: &str, _values: ...) -> String {
    String::new()
}
fn print_version(_program_name: &str, _level: u32) {}

const PROGRAM_NAME: &str = "irontide";
const PROGRAM_URL: &str = "https://irontide.io/";
const LICENSE_str: &str = include_str!("LICENSE");

// Dummy implementation for missing functions
extern "C" {
    fn rs_setup_human_panic();
    fn utils_initialize_ssl_implementation();
    fn setlocale(_lc_type: libc::c_int, _locale: *const libc::c_char);
    fn bindtextdomain(_package: *const libc::c_char, _localedir: *const libc::c_char);
    fn bind_textdomain_codeset(_package: *const libc::c_char, _codeset: *const libc::c_char);
    fn textdomain(_domainname: *const libc::c_char);
    fn curses_version() -> *const libc::c_char;
    fn curl_version() -> *const libc::c_char;
    fn sqlite3_libversion() -> *const libc::c_char;
}

use std::ffi::CString;

fn main() {
    unsafe {
        rs_setup_human_panic();
        utils_initialize_ssl_implementation();

        let locale = CString::new("en_US.UTF-8").unwrap();
        setlocale(libc::LC_CTYPE, locale.as_ptr());
        setlocale(libc::LC_MESSAGES, locale.as_ptr());

        textdomain(PACKAGE);
        bindtextdomain(PACKAGE, LOCALEDIR);
        bind_textdomain_codeset(PACKAGE, "UTF-8");

        rsspp::Parser::global_init();

        let configpaths = ConfigPaths::new();
        if !configpaths.initialized() {
            eprintln!("Error: {}", configpaths.error_message());
            std::process::exit(EXIT_FAILURE);
        }

        let mut c = Controller::new(configpaths);
        let mut v = View;
        c.set_view(&v);

        let args = CliArgsParser;
        configpaths.process_args(args);

        if args.should_print_usage() {
            print_usage(args.program_name(), configpaths);
            if args.return_code().is_some() {
                std::process::exit(args.return_code().unwrap());
            }
        } else if args.show_version() {
            print_version(PROGRAM_NAME, 1);
            std::process::exit(EXIT_SUCCESS);
        }

        let ret = c.run(args);
        rsspp::Parser::global_cleanup();
        std::process::exit(ret);
    }
}

// Dummy implementation for missing structs and functions
struct ConfigPaths;
impl ConfigPaths {
    fn initialized(&self) -> bool {
        true
    }
    fn error_message(&self) -> String {
        String::new()
    }
    fn process_args(&self, _args: CliArgsParser) {}
    fn config_file(&self) -> &str {
        "config.txt"
    }
    fn url_file(&self) -> &str {
        "urls.txt"
    }
    fn cache_file(&self) -> &str {
        "cache.db"
    }
    fn queue_file(&self) -> &str {
        "queue.txt"
    }
    fn search_history_file(&self) -> &str {
        "search_history.txt"
    }
    fn cmdline_history_file(&self) -> &str {
        "cmdline_history.txt"
    }
}

struct CliArgsParser;
impl CliArgsParser {
    fn program_name(&self) -> &str {
        "irontide"
    }
    fn should_print_usage(&self) -> bool {
        false
    }
    fn return_code(&self) -> Option<i32> {
        None
    }
    fn show_version(&self) -> u32 {
        0
    }
}

struct View;

// Dummy implementation for missing structs and functions
const PACKAGE: &str = "irontide";
const LOCALEDIR: &str = "/usr/share/locale";
