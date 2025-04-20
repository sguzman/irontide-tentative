// src/main.rs
// Entry point for Irontide (Rust port of Newsboat)
extern crate clap;
extern crate gettext;
extern crate libc;
extern crate ncurses;
extern crate openssl;
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
use libc::{uname, utsname};
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
        // TODO: print copyright and bundled library info
        let mut uts: utsname = unsafe { std::mem::zeroed() };
        unsafe { uname(&mut uts) };
        // Convert uts.sysname etc. to Rust strings as needed
    } else {
        println!("{}", xlicense::LICENSE_STR);
    }
}

fn main() {
    // Set up panic hook for human-readable Rust panics
    unsafe { rs_setup_human_panic() };

    // Initialize SSL (OpenSSL or other)
    utils::initialize_ssl_implementation();

    // Localization setup
    gettext::bind_textdomain_codeset(xlicense::PACKAGE, "UTF-8");
    gettext::textdomain(xlicense::PACKAGE);

    // Initialize RSS parser global state
    parser::Parser::global_init();

    // Load configuration file paths
    let configpaths = configpaths::ConfigPaths::new();
    if !configpaths.initialized() {
        eprintln!("{}", configpaths.error_message());
        process::exit(1);
    }

    // Set up controller and view
    let mut controller = controller::Controller::new(&configpaths);
    let view = view::View::new(&controller);
    controller.set_view(&view);

    // Parse command-line arguments
    let args = cliargsparser::CliArgsParser::parse(env::args());
    configpaths.process_args(&args);

    // Handle --help or --version
    if args.should_print_usage() {
        print_usage(&args.program_name(), &configpaths);
        if let Some(code) = args.return_code() {
            process::exit(code);
        }
    } else if let Some(level) = args.show_version() {
        print_version(&args.program_name(), level);
        process::exit(0);
    }

    // Run the main controller loop
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

    // Clean up RSS parser global state
    parser::Parser::global_cleanup();

    process::exit(exit_code);
}
