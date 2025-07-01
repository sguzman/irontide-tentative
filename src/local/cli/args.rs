// CLI arguments struct, derived from C++ options in newsboat.cpp
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser, Clone, PartialEq, Eq)]
#[clap(author = "Salvador Guzman")]
#[clap(name = "irontide", version = "0.1.0", about = "Rust port of Newsboat")]
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
