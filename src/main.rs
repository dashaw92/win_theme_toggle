#![windows_subsystem = "windows"]

use std::error::Error;
use std::path::PathBuf;
use std::{env, thread};

mod app;
mod config;
mod reg;

use app::Message;
use reg::Theme;
use tray_item::TrayItem;

use crate::app::AppMode;
use crate::config::Config;

pub(crate) type UnitResult = Result<(), Box<dyn Error>>;

fn main() -> UnitResult {
    let (app_tx, app_rx) = crossbeam_channel::unbounded();
    let (main_tx, main_rx) = crossbeam_channel::unbounded();

    let arg: String = env::args().skip(1).take(1).collect();
    let mode = match arg.trim().to_lowercase().as_str() {
        cmd @ "dark" | cmd @ "light" => AppMode::ForceTheme(cmd.into()),
        "auto" => AppMode::Auto(Config::from_cfg(get_config_file())?),
        _ => {
            let config = Config::from_cfg(get_config_file())?;
            AppMode::Daemon(config, app_rx)
        }
    };

    thread::spawn(|| {
        let _ = app::launch(mode);
    });

    let mut tray = TrayItem::new("Win Theme Toggle", "wtt-icon")?;
    let tray = tray.inner_mut();
    tray.add_label("Win Theme Toggle")?;

    let tx = app_tx.clone();
    tray.add_menu_item("Dark", move || {
        tx.send(Message::Override(Some(Theme::Dark)))
            .expect("Failed to send override command!");
    })?;

    let tx = app_tx.clone();
    tray.add_menu_item("Light", move || {
        tx.send(Message::Override(Some(Theme::Light)))
            .expect("Failed to send override command!");
    })?;

    let tx = app_tx.clone();
    tray.add_menu_item("Auto", move || {
        tx.send(Message::Override(None))
            .expect("Failed to send override command!");
    })?;

    tray.add_separator()?;

    // let tx = app_tx.clone();
    tray.add_menu_item("Quit", move || {
        app_tx
            .send(Message::Shutdown)
            .expect("Failed to send shutdown message!");
        main_tx.send(()).expect("Failed to send shutdown message!");
    })?;
    loop {
        if let Ok(()) = main_rx.recv() {
            break;
        }
    }
    Ok(())
}

pub(crate) fn get_config_file() -> PathBuf {
    let mut pwd = env::current_exe()
        .expect("Failed to get current dir.")
        .parent()
        .expect("Call to parent() failed")
        .to_path_buf();
    pwd.push("config.toml");

    pwd
}
