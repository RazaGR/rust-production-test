mod commands;
use clap::{Arg, Command};
use dotenv::dotenv;

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let mut command = Command::new("CLI application")
        .version("1.0")
        .author("Mohsin Raza <contact@raza.gr>")
        .about("cli utility to manage application")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );

    command = commands::configure(command);

    let matches = command.get_matches();

    commands::handle(&matches)?;

    Ok(())
}
