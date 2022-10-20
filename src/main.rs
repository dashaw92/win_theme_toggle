use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

mod config;
mod reg;

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
        "auto" => {
            let config = Config::from_cfg(get_config_file())?;
            return reg::set_theme(get_theme(&config));
        }
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
    let (_tx, rx) = mpsc::channel::<()>();
    service_impl(rx)
}

fn service_impl(rx: Receiver<()>) -> Result<(), Box<dyn Error>> {
    let config = Config::from_cfg(get_config_file())?;
    let mut last = get_theme(&config);
    let mut now;

    reg::set_theme(last)?;
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => return Err("Got termination signal, shutting down...".into()),
            Err(mpsc::RecvTimeoutError::Disconnected) => return Err("Channel is broken!".into()),
            _ => {}
        }

        now = get_theme(&config);

        if now == last {
            continue;
        }

        println!(
            "Checking for auto-theme change: Last: {:?}, now: {:?}",
            &last, &now
        );
        reg::set_theme(now)?;
        last = now;
    }
}
