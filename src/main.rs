#![windows_subsystem = "windows"]

use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, thread};

mod config;
mod reg;

use reg::Theme;

use crate::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Parsing args.");
    let arg: String = env::args().skip(1).take(1).collect();
    match arg.trim().to_lowercase().as_str() {
        "" => {}
        cmd @ "dark" | cmd @ "light" => {
            println!("Found force flag, setting to \"{}\"", cmd);
            reg::set_theme(cmd.into())?;
            return Ok(());
        }
        "service" => {
            println!("Found service flag, running as a service.");
            let config = Config::from_cfg(get_config_file())?;
            let mut last = get_theme(&config);
            let mut now;

            reg::set_theme(last)?;
            loop {
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
                thread::sleep(Duration::from_secs(1));
            }
        }
        _ => {
            eprintln!("Unknown command. Expected either \"dark\", \"light\", \"service\", or no args at all. Ignoring...");
        }
    }

    let config = Config::from_cfg(get_config_file())?;
    println!("Setting the theme.");
    reg::set_theme(if config.is_light_time() {
        reg::Theme::Light
    } else {
        reg::Theme::Dark
    })
}

fn get_config_file() -> PathBuf {
    let mut pwd = env::current_exe()
        .expect("Failed to get current dir.")
        .parent()
        .expect("Call to parent() failed")
        .to_path_buf();
    pwd.push("config.toml");

    println!("Config is located at: {:?}", pwd);
    pwd
}

fn get_theme(config: &Config) -> Theme {
    if config.is_light_time() {
        reg::Theme::Light
    } else {
        reg::Theme::Dark
    }
}
