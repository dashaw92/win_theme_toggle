use std::env;
use std::error::Error;
use std::path::PathBuf;

mod config;
mod reg;

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
        _ => {
            eprintln!("Unknown command. Expected either \"dark\" or \"light\", or no args at all. Ignoring...");
        }
    }

    println!("Loading config.");
    let config = Config::from_cfg(get_config_file())?;
    println!("Setting the theme.");
    reg::set_theme(if config.is_light_time() {
        reg::Theme::Light
    } else {
        reg::Theme::Dark
    })
}

fn get_config_file() -> PathBuf {
    let mut pwd = env::current_dir().expect("Failed to get current dir.");
    pwd.push("config.toml");
    pwd
}
