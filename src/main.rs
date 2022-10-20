#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, thread};

mod app;
mod config;
mod reg;
mod tray;

use app::{AppMode, Message};
use notify::{RecursiveMode, Watcher};

use crate::config::Config;

pub(crate) type UnitResult = Result<(), Box<dyn Error>>;

fn main() -> UnitResult {
    //Sends messages to app.rs
    let (app_tx, app_rx) = crossbeam_channel::unbounded();
    //Messages for the tray <-> app
    let (main_tx, main_rx) = crossbeam_channel::unbounded();
    //Messages between the config file watcher and main
    let (cfg_tx, cfg_rx) = crossbeam_channel::unbounded();

    let arg: String = env::args().skip(1).take(1).collect();
    let mode = match arg.trim().to_lowercase().as_str() {
        cmd @ "dark" | cmd @ "light" => AppMode::ForceTheme(cmd.into()),
        "auto" => AppMode::Auto(Config::from_cfg(get_config_file())?),
        _ => {
            let config = Config::from_cfg(get_config_file())?;
            AppMode::Daemon(config, app_rx)
        }
    };

    let app_handle = thread::spawn(|| {
        let _ = dbg!(app::launch(mode));
    });

    let app_tx_clone = app_tx.clone();
    let tray_handle = thread::spawn(move || {
        let _ = dbg!(tray::start(app_tx_clone, main_tx, main_rx));
    });

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            dbg!(&res);
            if let Ok(event) = res {
                if event.kind.is_modify() {
                    cfg_tx.send(()).expect("Failed to send update message.");
                }
            }
        })?;

    watcher.watch(get_config_file().as_ref(), RecursiveMode::NonRecursive)?;

    loop {
        if let Ok(()) = cfg_rx.recv_timeout(Duration::from_millis(500)) {
            let new_config = Config::from_cfg(get_config_file())?;
            app_tx.send(Message::UpdateConfig(new_config))?;
        }

        if app_handle.is_finished() || tray_handle.is_finished() {
            return Ok(());
        }
    }
}

pub(crate) fn get_config_file() -> PathBuf {
    let mut pwd = env::current_exe()
        .expect("Failed to get current dir.")
        .parent()
        .expect("Call to parent() failed")
        .to_path_buf();
    pwd.push("config.toml");

    dbg!(pwd)
}
