use clap::{Arg, Command};
use cli::{commands, settings};
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

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "CLI")?;

    // as_deref() is used to convert the Option<String> to Option<&str>, which allows borrowing the value without taking ownership.
    println!(
        "db url: {}",
        settings
            .database
            .url
            .as_deref()
            .unwrap_or("missing database url")
    );

    println!(
        "log level: {}",
        &settings.logging.log_level.as_deref().unwrap_or("info")
    );
    commands::handle(&matches, &settings)?;

    Ok(())
}
