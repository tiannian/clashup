use clap::{Parser, Subcommand};

use crate::Result;

mod client;
mod config;
mod daemon;
mod init;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Command {
    /// Config dir path
    #[clap(short, long)]
    pub config_dir: Option<String>,

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Daemon(daemon::Daemon),
    Config(config::Config),
    Client(client::Client),
    Init(init::Init),
}

impl Command {
    pub async fn execute(&mut self) -> Result<()> {
        match &mut self.subcommand {
            SubCommand::Daemon(c) => c.execute(self.config_dir.as_deref()).await?,
            SubCommand::Config(c) => c.execute(self.config_dir.as_deref()).await?,
            SubCommand::Client(c) => c.execute().await?,
            SubCommand::Init(c) => c.execute(self.config_dir.as_deref()).await?,
        }

        Ok(())
    }
}
