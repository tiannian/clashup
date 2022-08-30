use clap::Args;

use crate::Result;

#[derive(Args, Debug)]
pub struct Client {
    /// Upstream of daemon
    #[clap(short, long)]
    pub upstream: Option<String>,

    /// Set value
    #[clap(short, long)]
    pub set: Option<String>,

    /// Del value
    #[clap(short, long)]
    pub del: Option<String>,
}

impl Client {
    pub async fn execute(&mut self) -> Result<()> {
        Ok(())
    }
}

