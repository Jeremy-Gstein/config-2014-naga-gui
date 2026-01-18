use eframe::egui;
use std::sync::{Arc, Mutex};

mod app;
mod tray;
mod state;
mod tabs;

use app::ConfigNagaGuiApp;
use state::AppState;

fn main() -> Result<(), eframe::Error> {
    let app_state = Arc::new(Mutex::new(AppState {
        //window_visible: true,
        remapping_enabled: false,
    }));

    std::thread::spawn({
        let tray_state = app_state.clone();
        move || tray::setup_system_tray(tray_state)
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Config 2014 Naga GUI",
        options,
        Box::new(|_| Ok(Box::new(ConfigNagaGuiApp::new(app_state)))),
    )
}
