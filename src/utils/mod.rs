mod update_config;
pub use update_config::*;

mod start_clash;
pub use start_clash::*;

mod download_clash;
pub use download_clash::*;

pub fn default_dir() -> String {
    let home = env!("HOME");

    format!("{}/.config/clash", home)
}
