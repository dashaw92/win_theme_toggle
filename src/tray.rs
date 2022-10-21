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

    tray.add_menu_item("Dark", build_override(app_tx.clone(), Some(Theme::Dark)))?;
    tray.add_menu_item("Light", build_override(app_tx.clone(), Some(Theme::Light)))?;
    tray.add_menu_item("Auto", build_override(app_tx.clone(), None))?;

    tray.add_separator()?;

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

fn build_override(tx: Sender<Message>, theme: Option<Theme>) -> impl Fn() {
    send_message(tx, Message::Override(theme))
}

fn send_message(tx: Sender<Message>, msg: Message) -> impl Fn() {
    move || {
        tx.send(msg.clone()).expect("Failed to send message!");
    }
}
