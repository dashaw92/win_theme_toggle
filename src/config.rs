use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use crate::{debug, error::WttError, WttResult};

use chrono::{Local, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Config {
    light_time: NaiveTime,
    dark_time: NaiveTime,
}

//Defaults:
//Light time switch at 7:30 AM
//Dark time switch at 5:30 PM
impl Default for Config {
    fn default() -> Self {
        Self {
            light_time: NaiveTime::from_hms(7, 0, 0),
            dark_time: NaiveTime::from_hms(17, 30, 0),
        }
    }
}

impl Config {
    pub(crate) fn from_cfg<P: AsRef<Path>>(config: P) -> WttResult<Self> {
        debug!("Loading config file");
        let config = match read_to_string(&config) {
            Ok(file) => toml::from_str(&file).map_err(|_| WttError::ConfigDeserialize)?,
            Err(_) => {
                debug!("Config file does not exist, attempting to create one.");
                let mut config = File::create(config)?;
                let default =
                    toml::to_string(&Config::default()).map_err(|_| WttError::ConfigDeserialize)?;

                if let Err(e) = config.write_all(default.as_bytes()) {
                    debug!("Failed to write new config file: {:?}", e);
                    return Err(WttError::ConfigLoad(e));
                }

                Config::default()
            }
        };

        if config.light_time == config.dark_time {
            return Err(WttError::ConfigInvalid);
        }

        Ok(config)
    }

    pub(crate) fn is_light_time(&self) -> bool {
        let now = Local::now().time();
        now >= self.light_time && now < self.dark_time
    }
}
