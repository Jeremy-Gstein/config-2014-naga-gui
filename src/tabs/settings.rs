use eframe::egui;
use std::process::Command;
use crate::app::ConfigNagaGuiApp;

pub fn show(app: &mut ConfigNagaGuiApp, ui: &mut egui::Ui) {
    ui.heading("⚙ Settings");
    ui.add_space(10.0);

    ui.label("Config directory:");
    ui.horizontal(|ui| {
        ui.label(app.config_dir.to_string_lossy().to_string());
        if ui.button("📋 Copy").clicked() {
            ui.output_mut(|o| {
                o.copied_text = app.config_dir.to_string_lossy().to_string();
            });
            app.status_message = "Path copied to clipboard".to_string();
        }
    });

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(10.0);

    ui.label("Installation check:");
    if ui.button("Check if config-2014-naga is installed").clicked() {
        match Command::new("which").arg("config-2014-naga").output() {
            Ok(output) if output.status.success() => {
                let path = String::from_utf8_lossy(&output.stdout);
                app.status_message =
                    format!("✓ config-2014-naga found at: {}", path.trim());
            }
            _ => {
                app.status_message =
                    "✗ config-2014-naga not found. Install with: cargo install config-2014-naga"
                        .to_string();
            }
        }
    }

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(10.0);

    if let Some(path) = &app.loaded_config_path {
        ui.label("Currently loaded config:");
        ui.label(egui::RichText::new(path.display().to_string()).monospace());
        ui.add_space(10.0);

        if ui.button("🗑 Clear (Use Default)").clicked() {
            app.loaded_config_path = None;
            app.key_mappings.clear();
            app.status_message = "Cleared config - will use default".to_string();
        }
    }
}
