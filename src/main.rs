use std::env;
use std::error::Error;
use std::path::PathBuf;

mod config;
mod reg;
mod service;

use crate::config::Config;
use crate::reg::Theme;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Parsing args.");
    let arg: String = env::args().skip(1).take(1).collect();
    match arg.trim().to_lowercase().as_str() {
        cmd @ "dark" | cmd @ "light" => {
            println!("Found force flag, setting to \"{}\"", cmd);
            reg::set_theme(cmd.into())?;
            return Ok(());
        }
        // "" => {
        //     let config = Config::from_cfg(get_config_file())?;
        //     return reg::set_theme(get_theme(&config));
        // }
        _ => {}
    }
    run_service()
}

pub(crate) fn get_config_file() -> PathBuf {
    let mut pwd = env::current_exe()
        .expect("Failed to get current dir.")
        .parent()
        .expect("Call to parent() failed")
        .to_path_buf();
    pwd.push("config.toml");

    println!("Config is located at: {:?}", pwd);
    pwd
}

pub(crate) fn get_theme(config: &Config) -> Theme {
    if config.is_light_time() {
        Theme::Light
    } else {
        Theme::Dark
    }
}

fn run_service() -> Result<(), Box<dyn Error>> {
    service::start_service().map_err(|_| "Failed to run service".into())
}
