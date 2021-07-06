use ruic::{Config, Ruic};
use std::fs::File;
use std::io;
use std::process::Command;
use structopt::StructOpt;

fn main() -> Result<(), String> {
    try_main().map_err(|x| x.to_string())
}

fn try_main() -> io::Result<()> {
    let mut config = Config::from_args();
    let should_format = config.format;
    let path = config
        .out
        .take()
        .unwrap_or_else(|| config.path.join("uic.rs"));
    let file = File::create(&path)?;
    Ruic::new(file, config).process()?;
    if should_format {
        let status = Command::new("rustfmt").arg(&path).status()?;
        if !status.success() {
            return Err(match status.code() {
                Some(code) => io::Error::from_raw_os_error(code),
                None => io::Error::new(io::ErrorKind::Interrupted, "process terminated"),
            });
        }
    }
    Ok(())
}
