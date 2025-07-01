use clap::Parser;
use irontide::local::cli::args::CliArgs;
use std::path::PathBuf;

#[test]
fn unknown_option_is_error() {
    let res = CliArgs::try_parse_from(["irontide", "--bad-option"]);
    assert!(res.is_err());
}

#[test]
fn import_from_opml_parses() {
    let args = CliArgs::try_parse_from(["irontide", "-i", "feeds.opml"]).unwrap();
    assert_eq!(args.import_from_opml, Some(PathBuf::from("feeds.opml")));
}

#[test]
fn refresh_on_start_flag() {
    let args = CliArgs::try_parse_from(["irontide", "--refresh-on-start"]).unwrap();
    assert!(args.refresh_on_start);
}

#[test]
fn export_to_opml_flag() {
    let args = CliArgs::try_parse_from(["irontide", "--export-to-opml"]).unwrap();
    assert!(args.export_to_opml);
}

#[test]
fn url_file_option() {
    let args = CliArgs::try_parse_from(["irontide", "--url-file", "urls.txt"]).unwrap();
    assert_eq!(args.url_file, Some(PathBuf::from("urls.txt")));
}

#[test]
fn log_file_option() {
    let args = CliArgs::try_parse_from(["irontide", "--log-file", "app.log"]).unwrap();
    assert_eq!(args.log_file, Some(PathBuf::from("app.log")));
}

#[test]
fn log_level_option() {
    let args = CliArgs::try_parse_from(["irontide", "--log-level", "5"]).unwrap();
    assert_eq!(args.log_level, Some(5));
}

#[test]
fn log_level_out_of_range() {
    assert!(CliArgs::try_parse_from(["irontide", "--log-level", "0"]).is_err());
    assert!(CliArgs::try_parse_from(["irontide", "--log-level", "7"]).is_err());
}
