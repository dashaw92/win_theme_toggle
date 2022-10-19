use std::{
    error::Error,
    ffi::OsString,
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};

use crate::{config::Config, get_config_file, get_theme, reg};

define_windows_service!(ffi_main, service_entry);

pub(crate) fn start_service() -> Result<(), windows_service::Error> {
    service_dispatcher::start("win_theme_toggle", ffi_main)
}

fn service_entry(args: Vec<OsString>) {
    if let Err(_e) = run_service(args) {}
}

fn run_service(_: Vec<OsString>) -> Result<(), windows_service::Error> {
    let (tx, rx) = mpsc::channel::<()>();

    let events = move |event| -> ServiceControlHandlerResult {
        match event {
            ServiceControl::Stop => {
                tx.send(()).expect("Failed to send on channel");
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status = service_control_handler::register("win_theme_toggle", events)?;

    status.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    service_impl(rx).expect("Service encountered an error.");

    status.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;
    Ok(())
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
