use tokio::process::Command;

use crate::{Error, Result};

pub async fn start_clash(dir: Option<&str>) -> Result<()> {
    let mut child = Command::new("clash");

    if let Some(dir) = dir {
        child.arg("-d").arg(dir);
    }

    let mut child = child.spawn()?;

    let status = child.wait().await?;

    if !status.success() {
        return Err(Error::ExitCodeError);
    }

    Ok(())
}
