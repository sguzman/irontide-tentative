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
mod cliargsparser;
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

use clap::Parser as ClapParser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use libc::{uname, utsname};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::{env, process};

extern "C" {
    fn rs_setup_human_panic();
}

fn print_usage(argv0: &str, configpaths: &configpaths::ConfigPaths) {
    let msg = format!(
        "{} {}\nusage: {} [-i <file>|-e] [-u <urlfile>] \
         [-c <cachefile>] [-x <command> ...] [-h]\n",
        xlicense::PROGRAM_NAME,
        utils::program_version(),
        argv0
    );
    println!("{}", msg);
    // TODO: replicate help listing logic
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
        // TODO: convert uts fields to strings
    } else {
        println!("{}", xlicense::LICENSE_STR);
    }
}

fn main() {
    // Setup panic hook
    unsafe { rs_setup_human_panic() };

    // Initialize SSL
    utils::initialize_ssl_implementation();

    // Localization setup
    gettext::bind_textdomain_codeset(xlicense::PACKAGE, "UTF-8");
    gettext::textdomain(xlicense::PACKAGE);

    // RSS parser global init
    parser::Parser::global_init();

    // Config paths
    let configpaths = configpaths::ConfigPaths::new();
    if !configpaths.initialized() {
        eprintln!("{}", configpaths.error_message());
        process::exit(1);
    }

    // Controller and view
    let mut controller = controller::Controller::new(&configpaths);
    let view = view::View::new(&controller);

    // Setup terminal with ratatui
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    controller.set_view(&view);
    let args = cliargsparser::CliArgsParser::parse(env::args());
    configpaths.process_args(&args);

    // Help/version handling
    if args.should_print_usage() {
        print_usage(&args.program_name(), &configpaths);
        if let Some(code) = args.return_code() {
            process::exit(code);
        }
    } else if let Some(level) = args.show_version() {
        print_version(&args.program_name(), level);
        process::exit(0);
    }

    // Main application loop
    let exit_code = match controller.run(&args) {
        Ok(code) => code,
        Err(dbexception::DbException(e)) => {
            stflpp::Stfl::reset();
            eprintln!("Caught DbException with message: {}", e);
            process::exit(1);
        }
        Err(matcherexception::MatcherException(e)) => {
            stflpp::Stfl::reset();
            eprintln!("Caught MatcherException with message: {}", e);
            process::exit(1);
        }
        Err(exception::Exception(e)) => {
            stflpp::Stfl::reset();
            eprintln!("Caught Exception with message: {}", e);
            process::exit(1);
        }
    };

    // Cleanup
    parser::Parser::global_cleanup();

    // Restore terminal
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen).expect("Failed to leave alternate screen");
    disable_raw_mode().expect("Failed to disable raw mode");

    process::exit(exit_code);
}
