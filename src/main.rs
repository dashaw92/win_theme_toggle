#![windows_subsystem = "windows"]

use std::fs::OpenOptions;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, thread};

mod app;
mod config;
mod error;
mod reg;
mod tray;

use app::{AppMode, Message};
use error::WttError;
pub(crate) use log::debug;
use notify::{RecursiveMode, Watcher};
use simplelog::*;

use crate::config::Config;

pub(crate) type WttResult<T> = Result<T, WttError>;

fn main() -> WttResult<()> {
    WriteLogger::init(
        LevelFilter::Trace,
        simplelog::Config::default(),
        OpenOptions::new()
            .append(true)
            .create(true)
            .open("wtt.log")
            .expect("Failed to open log file"),
    )?;

    debug!("{}", "-".repeat(80));
    debug!("Logger has been setup!");

    let config_file = get_config_file();

    //Sends messages to app.rs
    let (app_tx, app_rx) = crossbeam_channel::unbounded();
    //Messages for the tray <-> app
    let (main_tx, main_rx) = crossbeam_channel::unbounded();
    //Messages between the config file watcher and main
    let (cfg_tx, cfg_rx) = crossbeam_channel::unbounded();

    let arg: String = env::args().skip(1).take(1).collect();
    let mode = match arg.trim().to_lowercase().as_str() {
        cmd @ "dark" | cmd @ "light" => AppMode::ForceTheme(cmd.into()),
        "auto" => AppMode::Auto(Config::from_cfg(&config_file)?),
        _ => {
            let config = Config::from_cfg(&config_file)?;
            AppMode::Daemon(config, app_rx)
        }
    };

    let app_handle = thread::spawn(|| {
        let out = app::launch(mode);
        debug!("app::launch() = {:?}", out);
        out
    });

    let app_tx_clone = app_tx.clone();
    let tray_handle = thread::spawn(move || {
        let out = tray::start(app_tx_clone, main_tx, main_rx);
        debug!("tray::start() = {:?}", out);
        out
    });

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            debug!("{:?}", &res);
            if let Ok(event) = res {
                if event.kind.is_modify() {
                    cfg_tx.send(()).expect("Failed to send update message.");
                }
            }
        })?;

    watcher.watch(config_file.as_ref(), RecursiveMode::NonRecursive)?;

    loop {
        if cfg_rx.recv_timeout(Duration::from_millis(500)).is_ok() {
            let new_config = Config::from_cfg(get_config_file())?;
            app_tx
                .send(Message::UpdateConfig(new_config))
                .expect("Failed to send an app message.");
        }

        if app_handle.is_finished() || tray_handle.is_finished() {
            debug!("Process ended gracefully.");
            debug!("{}", "-".repeat(80));
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

    debug!("{:?}", pwd);
    pwd
}
