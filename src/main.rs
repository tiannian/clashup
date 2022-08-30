use clap::Parser;

mod command;
mod utils;

mod error;
pub use error::*;

#[tokio::main]
async fn main() {
    let mut command = command::Command::parse();

    command.execute().await.unwrap();
}
