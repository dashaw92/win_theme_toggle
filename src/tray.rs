use crate::app::Message;
use crate::reg::Theme;
use crate::WttResult;
use crossbeam_channel::{Receiver, Sender};
use tray_item::TrayItem;

pub(crate) fn start(
    app_tx: Sender<Message>,
    main_tx: Sender<()>,
    main_rx: Receiver<()>,
) -> WttResult {
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
            return Ok("Got termination signal, disposing of tray icon.");
        }
    }
}
