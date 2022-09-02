use clap::Args;

use crate::{
    utils::{self, default_dir},
    Result,
};

#[derive(Args, Debug)]
pub struct Daemon {
    /// subscription url
    #[clap(short, long)]
    pub sub_url: Option<String>,
}

impl Daemon {
    pub async fn execute(&mut self, config_dir: Option<&str>) -> Result<()> {
        if let Some(url) = &self.sub_url {
            utils::update_config(config_dir, url).await?;
        }

        let config_dir = if let Some(d) = config_dir {
            String::from(d)
        } else {
            default_dir()
        };

        utils::start_clash(&config_dir).await?;

        Ok(())
    }
}
