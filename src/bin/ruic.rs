use ruic::{Config, Ruic};
use std::fs::File;
use std::io;
use structopt::StructOpt;

fn main() -> Result<(), String> {
    try_main().map_err(|x| x.to_string())
}

fn try_main() -> io::Result<()> {
    let config = Config::from_args();
    let file = match &config.out {
        Some(out) => File::create(out),
        None => File::create(config.path.join("uic.rs")),
    }?;
    Ruic::new(file, config).process()
}
