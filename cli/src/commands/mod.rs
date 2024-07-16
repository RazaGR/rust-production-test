mod createadmin;
mod hello;
mod migrate;
mod serve;

use clap::{ArgMatches, Command};

use crate::settings::Settings;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(createadmin::configure())
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, settings)?;
    serve::handle(matches, settings)?;
    migrate::handle(matches, settings)?;
    createadmin::handle(matches, settings)?;

    Ok(())
}
