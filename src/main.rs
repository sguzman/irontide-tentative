use env_logger::{Builder, Env};
use std::fmt::Write;

use colored::Colorize;

const PROGRAM_NAME: &str = "irontide";

fn setup_logging() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let timestamp = chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S%.3f")
                .to_string();
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".cyan(),
            };
            let message = record.args();
            Ok(println!(
                "{} [{}] {}: {:#?}",
                timestamp, level, message, buf
            ))
        })
        .init();

    log::info!("{}: Hello world", PROGRAM_NAME);
}

fn atexit() {
    // At exit, print a message
    static mut EXIT_MESSAGE: Option<String> = None;
    unsafe {
        EXIT_MESSAGE = Some(format!("{} stopped", PROGRAM_NAME));
        libc::atexit(exit_handler);
    }

    extern "C" fn exit_handler() {
        unsafe {
            if let Some(ref message) = EXIT_MESSAGE {
                log::info!("{}", message);
            }
        }
    }
}

fn init() {
    setup_logging();
    atexit();
}

fn main() {
    init();
    std::process::exit(0);
}
