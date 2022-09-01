use tokio::fs;

use crate::Result;

use super::default_dir;

pub async fn update_config(config: Option<&str>, url: &str) -> Result<()> {
    let config = if let Some(s) = config {
        format!("{}/config.yaml", s)
    } else {
        format!("{}/config.yaml", default_dir())
    };

    let config_resp = reqwest::get(url).await?;

    let config_str = config_resp.text().await?;

    fs::write(config, config_str).await?;
    Ok(())
}
