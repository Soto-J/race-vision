use tauri::{App, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn register_shortcuts(app: &App) -> Result<(), tauri_plugin_global_shortcut::Error> {
    let f6 = Shortcut::new(Some(Modifiers::CONTROL), Code::F6);

    app.global_shortcut()
        .on_shortcut(f6, move |app, shortcut, event| {
            if shortcut == &f6 && event.state() == ShortcutState::Pressed {
                if let Err(e) = app.emit("toggle-edit-mode", ()) {
                    eprintln!("Failed to emit toggle-edit-mode: {e:?}")
                };
            }
        })?;

    Ok(())
}
