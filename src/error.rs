use log::SetLoggerError;
use tray_item::TIError;

#[derive(thiserror::Error, Debug)]
pub(crate) enum WttError {
    #[error("Failed to create logger")]
    LoggerSetup(#[from] SetLoggerError),
    #[error("Failed to load config")]
    ConfigLoad(#[from] std::io::Error),
    #[error("Failed to deserialize config")]
    ConfigDeserialize,
    #[error("Light and dark times cannot be the same")]
    ConfigInvalid,
    #[error("Failed to setup config file hot-reloading")]
    ConfigWatcherSetup(#[from] notify::Error),
    #[error("Tray error")]
    TrayError(#[from] TIError),
}
