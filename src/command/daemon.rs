use clap::Args;

use crate::{utils, Result};

#[derive(Args, Debug)]
pub struct Daemon {
    /// Config dir's url
    #[clap(short, long)]
    pub config_dir: Option<String>,

    /// subscription url
    #[clap(short, long)]
    pub sub_url: Option<String>,
}

impl Daemon {
    pub async fn execute(&mut self) -> Result<()> {
        if let Some(url) = &self.sub_url {
            let config = if let Some(cd) = &self.config_dir {
                Some(format!("{}/config.yaml", cd))
            } else {
                None
            };
            utils::update_config(config.as_deref(), url).await?;
        }

        utils::start_clash(self.config_dir.as_deref()).await?;

        Ok(())
    }
}
