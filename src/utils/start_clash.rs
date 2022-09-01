use std::path::Path;

use tokio::process::Command;

use crate::{Error, Result};

pub async fn start_clash(dir: &str) -> Result<()> {

    let p = format!("{}/clash", dir);

    if Path::new(&p).exists() {
        let mut child = Command::new(p);

        child.arg("-d").arg(dir);

        let mut child = child.spawn()?;

        let status = child.wait().await?;

        if !status.success() {
            return Err(Error::ExitCodeError);
        }

        Ok(())
    } else {
        Err(Error::NoClashBiniar)
    }
}
