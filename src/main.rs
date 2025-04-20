extern crate clap;
extern crate log;
extern crate reqwest;
extern crate rusqlite;

const PROGRAM_NAME: &str = "irontide";
const PACKAGE: &str = "irontide";
const LOCALEDIR: &str = "/usr/share/locale";

fn main() {
    let matches = CliArgs::parse();

    std::process::exit(ret);
}
