extern crate clap;
extern crate log;
extern crate reqwest;
extern crate rusqlite;

const PROGRAM_NAME: &str = "irontide";
const PACKAGE: &str = "irontide";
const LOCALEDIR: &str = "/usr/share/locale";

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
            write!(buf, "{} [{}] {}", timestamp, level, message)
        })
        .init();
}

fn main() {
    let matches = CliArgs::parse();

    std::process::exit(ret);
}
