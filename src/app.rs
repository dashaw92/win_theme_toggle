use crossbeam_channel::Receiver;

use self::AppMode::*;
use std::{error::Error, time::Duration};

use crate::{
    config::Config,
    debug,
    reg::{self, Theme},
    WttResult,
};

pub(crate) enum AppMode {
    ForceTheme(Theme),
    Auto(Config),
    Daemon(Config, Receiver<Message>),
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Shutdown,
    Override(Option<Theme>),
    UpdateConfig(Config),
}

pub(crate) fn launch(mode: AppMode) -> WttResult {
    let (config, rx) = match mode {
        ForceTheme(theme) => return reg::set_theme(theme),
        Auto(config) => return reg::set_theme(get_theme(&config)),
        Daemon(config, rx) => (config, rx),
    };

    impl_daemon(config, rx)
}

fn get_theme(config: &Config) -> Theme {
    if config.is_light_time() {
        Theme::Light
    } else {
        Theme::Dark
    }
}

fn impl_daemon(mut config: Config, rx: Receiver<Message>) -> Result<&'static str, Box<dyn Error>> {
    enum InnerMode {
        Auto,
        Force(Theme),
    }

    use InnerMode::*;

    let mut last = get_theme(&config);
    let mut now;

    let mut mode = InnerMode::Auto;

    reg::set_theme(last)?;
    loop {
        let mut recv = rx.recv_timeout(Duration::from_secs(1));
        if recv.is_ok() {
            debug!("app::impl(recv) = {:?}", &recv);
        }

        match recv {
            Ok(Message::Shutdown) => return Ok("Got termination signal, shutting down..."),
            Ok(Message::Override(theme)) => {
                debug!("Got an Override theme message!");
                if let Some(theme) = theme {
                    mode = Force(theme);
                } else {
                    mode = Auto;
                }
            }
            Ok(Message::UpdateConfig(ref mut new_config)) => {
                debug!("Got an UpdateConfig message, swapping out old config!");
                std::mem::swap(new_config, &mut config);
            }
            Err(_e) => {}
        }

        now = match mode {
            Force(theme) => theme,
            Auto => get_theme(&config),
        };

        if now == last {
            continue;
        }

        reg::set_theme(now)?;
        last = now;
    }
}
