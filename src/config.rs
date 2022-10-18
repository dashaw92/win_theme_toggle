use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use chrono::{Local, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    light_time: NaiveTime,
    dark_time: NaiveTime,
}

//Defaults:
//Light time switch at 7:30 AM
//Dark time switch at 6:00 PM
impl Default for Config {
    fn default() -> Self {
        Self {
            light_time: NaiveTime::from_hms(7, 0, 0),
            dark_time: NaiveTime::from_hms(17, 30, 0),
        }
    }
}

impl Config {
    pub(crate) fn from_cfg<P: AsRef<Path>>(config: P) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Loading config.");
        let config = match read_to_string(&config) {
            Ok(file) => toml::from_str(&file)?,
            Err(_) => {
                eprintln!("Config file not found. Attempting to create...");
                let mut config = File::create(config)?;
                let default = toml::to_string(&Config::default())?;

                if let Err(e) = config.write_all(default.as_bytes()) {
                    eprintln!("Could not create config file. Aborting.");
                    return Err(e.into());
                }

                Config::default()
            }
        };

        Ok(config)
    }

    pub(crate) fn is_light_time(&self) -> bool {
        let next_light = Local::today().and_time(self.light_time).unwrap();
        let next_dark = Local::today().and_time(self.dark_time).unwrap();

        let now = Local::now();
        now >= next_light && now < next_dark
    }
}
