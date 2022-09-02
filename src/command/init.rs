use clap::Args;

use crate::{
    utils::{default_dir, download_clash},
    Result,
};

#[derive(Args, Debug)]
pub struct Init {
    /// subscription url
    #[clap(short, long)]
    pub clash_release_url: Option<String>,

    /// Skip download clash binary
    #[clap(short, long)]
    pub skip_download_clash: bool,

    /// Path of clash binary
    #[clap(long)]
    pub clash_binary_path: Option<String>,
}

impl Init {
    pub async fn execute(&mut self, config_dir: Option<&str>) -> Result<()> {
        let config_dir = if let Some(c) = config_dir {
            String::from(c)
        } else {
            default_dir()
        };

        let clash_binary_path = if let Some(p) = &self.clash_release_url {
            p.clone()
        } else {
            format!("{}/clash", config_dir)
        };

        if !self.skip_download_clash {
            tokio::fs::create_dir_all(&config_dir).await?;

            let tar_path = format!("{}/clash.tar", config_dir);

            let clash_url = if let Some(ru) = &self.clash_release_url {
                ru.clone()
            } else {
                String::from(
                    "https://release.dreamacro.workers.dev/latest/clash-linux-amd64-v3-latest.gz",
                )
            };

            download_clash(&clash_url, &tar_path, &clash_binary_path).await?;
        }

        Ok(())
    }
}
