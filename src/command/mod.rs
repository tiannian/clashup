use clap::{Parser, Subcommand};

use crate::Result;

mod config;
mod daemon;
mod client;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Daemon(daemon::Daemon),
    Config(config::Config),
    Client(client::Client),
}

impl Command {
    pub async fn execute(&mut self) -> Result<()> {
        match &mut self.subcommand {
            SubCommand::Daemon(c) => c.execute().await?,
            SubCommand::Config(c) => c.execute().await?,
            SubCommand::Client(c) => c.execute().await?
        }

        Ok(())
    }
}
