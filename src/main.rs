extern crate clap;
extern crate log;
extern crate reqwest;
extern crate rusqlite;

const PROGRAM_NAME: &str = "irontide";
const PACKAGE: &str = "irontide";
const LOCALEDIR: &str = "/usr/share/locale";

fn main() {
    let matches = CliArgs::parse();

    let configpaths = ConfigPaths::new();
    if !configpaths.initialized() {
        eprintln!("Error: {}", configpaths.error_message());
        std::process::exit(EXIT_FAILURE);
    }

    std::process::exit(ret);
}
