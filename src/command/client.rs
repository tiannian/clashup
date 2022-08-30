use clap::Args;

use crate::Result;

#[derive(Args, Debug)]
pub struct Client {
    /// Upstream of daemon
    #[clap(short, long)]
    pub upstream: Option<String>,

    /// Key
    pub key: String,

    /// Value
    pub value: Option<String>,
}

impl Client {
    pub async fn execute(&mut self) -> Result<()> {
        Ok(())
    }
}

