use clap::Args;

use crate::{utils, Result};

#[derive(Args, Debug)]
pub struct Config {
    #[clap(short, long)]
    pub config_file: Option<String>,
    pub sub_url: String,
}

impl Config {
    pub async fn execute(&mut self) -> Result<()> {
        utils::update_config(self.config_file.as_deref(), &self.sub_url).await?;
        Ok(())
    }
}
