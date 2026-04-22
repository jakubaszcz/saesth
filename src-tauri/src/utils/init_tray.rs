use tauri::{menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, App, Manager};
pub fn init_tray(app: &mut App) {

    let quit = MenuItem::with_id(app, "quit", "Close", true, None::<&str>).unwrap();
    let show = MenuItem::with_id(app, "open", "Open", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&show, &quit]).unwrap();

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Saesth")
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "show" => {
                if let Some(w) = app.get_webview_window("main") {
                    w.show().unwrap();
                    w.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event {
                let app = tray.app_handle();
                if let Some(w) = app.get_webview_window("main") {
                    w.show().unwrap();
                    w.set_focus().unwrap();
                }
            }
        })
        .build(app).unwrap();
}