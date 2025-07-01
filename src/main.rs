use clap::{CommandFactory, Parser};
use colored::Colorize;
use env_logger::{Builder, Env};
use irontide::local::cli::args::CliArgs;
use std::path::Path;

const PROGRAM_NAME: &str = "irontide";

fn setup_logging(level: Option<u8>) {
    let mut builder = Builder::from_env(Env::default().default_filter_or("info"));
    if let Some(level) = level {
        let filter = match level {
            6 => log::LevelFilter::Debug,
            5 => log::LevelFilter::Info,
            4 => log::LevelFilter::Warn,
            _ => log::LevelFilter::Error,
        };
        builder.filter_level(filter);
    }
    builder
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

fn init(args: &CliArgs) {
    setup_logging(args.log_level);
    atexit();
}

fn main() {
    let args = CliArgs::parse();

    if args.help {
        CliArgs::command().print_help().unwrap();
        println!();
        return;
    }

    if args.version {
        println!("{} {}", PROGRAM_NAME, env!("CARGO_PKG_VERSION"));
        return;
    }

    init(&args);

    if let Some(url_file) = args.url_file.as_ref() {
        if let Err(e) = irontide::rss::process_urls_file(Path::new(url_file)) {
            eprintln!("Error processing URLs file: {e}");
        }
    }

    std::process::exit(0);
}
