use eframe::egui;
use crate::app::ConfigNagaGuiApp;

pub fn show(app: &mut ConfigNagaGuiApp, ui: &mut egui::Ui) {
    ui.heading("🎮 Key Mappings");
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        if app.is_remapping_active() {
            if ui.button("⏸ Stop").clicked() {
                app.stop_remapping();
            }
        } else if ui.button("▶ Start").clicked() {
            app.start_remapping();
        }

        ui.separator();

        if ui.button("📂 Load Config...").clicked() {
            app.browse_for_config();
        }

        if ui.button("💾 Save Config...").clicked() {
            app.save_config();
        }
    });

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(10.0);

    if let Some(path) = &app.loaded_config_path {
        ui.horizontal(|ui| {
            ui.label("📄 Loaded config:");
            ui.label(egui::RichText::new(path.display().to_string()).strong());
        });
        ui.add_space(10.0);
    } else {
        ui.colored_label(
            egui::Color32::LIGHT_BLUE,
            "ℹ No config loaded - using config-2014-naga default settings",
        );
        ui.add_space(10.0);
    }

    if !app.key_mappings.is_empty() {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("key_mappings_grid")
                .striped(true)
                .spacing([15.0, 10.0])
                .min_col_width(120.0)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("Naga Button").strong());
                    ui.label(egui::RichText::new("Maps To").strong());
                    ui.end_row();

                    for (button_num, key_value) in &mut app.key_mappings {
                        ui.label(format!("Button {}", button_num));

                        egui::ComboBox::from_id_salt(format!("button_{}", button_num))
                            .selected_text(key_value.as_str())
                            .width(200.0)
                            .show_ui(ui, |ui| {
                                for valid_key in &app.valid_keys {
                                    ui.selectable_value(
                                        key_value,
                                        valid_key.clone(),
                                        valid_key,
                                    );
                                }
                            });

                        ui.end_row();
                    }
                });
        });
    } else {
        ui.add_space(20.0);
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Load a config file to view and edit mappings");
                ui.add_space(10.0);
                if ui.button("📂 Browse for Config File").clicked() {
                    app.browse_for_config();
                }
            });
        });
    }
}
