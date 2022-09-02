use std::{
    fs::{File, Permissions},
    io::copy,
    os::unix::prelude::PermissionsExt,
    path::Path,
};

use crate::Result;

use flate2::read::GzDecoder;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

pub fn ungzip(from: &str, to: &str) -> Result<()> {
    let file = File::open(from)?;
    let mut gz = GzDecoder::new(file);

    let mut tof = File::create(to)?;

    copy(&mut gz, &mut tof)?;

    Ok(())
}

pub async fn download_clash(clash_url: &str, tar_path: &str, target: &str) -> Result<()> {
    println!("{}", clash_url);
    println!("{}", tar_path);
    println!("{}", target);

    let response = reqwest::get(clash_url).await?;

    if Path::new(tar_path).exists() {
        tokio::fs::remove_file(tar_path).await?;
    }

    if Path::new(target).exists() {
        tokio::fs::remove_file(target).await?;
    }

    let mut dest = tokio::fs::File::create(tar_path).await?;

    let mut response_stream = response.bytes_stream();

    while let Some(by) = response_stream.next().await {
        let by = by?;

        dest.write(&by).await?;
    }

    let from = String::from(tar_path);
    let to = String::from(target);

    let task = tokio::task::spawn_blocking(move || -> Result<()> {
        ungzip(&from, &to)?;
        Ok(())
    });

    task.await??;

    tokio::fs::set_permissions(target, Permissions::from_mode(0o744)).await?;

    Ok(())
}
