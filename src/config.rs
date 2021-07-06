use std::path::PathBuf;
use structopt::StructOpt;

/// Command-line arguments.
#[derive(Clone, Debug, Default, PartialEq, Eq, StructOpt)]
pub struct Config {
    /// Directory or file to scan
    #[structopt(default_value = ".")]
    pub path: PathBuf,
    /// Do not recursively scan directories
    #[structopt(long)]
    pub no_recursive: bool,
    /// Load objects that would ordinarily be ignored
    #[structopt(long)]
    pub all: bool,
    /// Output file [default: path + "/uic.rs"]
    #[structopt(long, short)]
    pub out: Option<PathBuf>,
    /// Suffix to append to widget names, e.g. "Ui" to turn "App" into "AppUi"
    #[structopt(long, short, default_value = "")]
    pub suffix: String,
    /// Run rustfmt on output
    #[structopt(long, short)]
    pub format: bool,
}
