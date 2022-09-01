use clap::Args;

use crate::{utils, Result};

#[derive(Args, Debug)]
pub struct Config {
    pub sub_url: String,
}

impl Config {
    pub async fn execute(&mut self, config_dir: Option<&str>) -> Result<()> {
        utils::update_config(config_dir, &self.sub_url).await?;
        Ok(())
    }
}
