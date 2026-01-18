use eframe::egui;
use crate::app::ConfigNagaGuiApp;

pub fn show(_app: &mut ConfigNagaGuiApp, ui: &mut egui::Ui) {
    ui.heading("ℹ About");
    ui.add_space(10.0);

    ui.label(
        egui::RichText::new("Config 2014 Naga Key Remapper")
            .size(16.0)
            .strong(),
    );
    ui.label("GUI Configuration Tool");

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    ui.label(
        "This tool provides a graphical interface for config-2014-naga, \
         which remaps Config 2014 Naga mouse buttons on Linux systems.",
    );

    ui.add_space(10.0);
    ui.hyperlink_to(
        "📦 View on crates.io",
        "https://crates.io/crates/config-2014-naga",
    );

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(10.0);

    ui.label(egui::RichText::new("Usage:").strong());
    ui.label("• Start with default: Click 'Start' with no config loaded");
    ui.label("• Load custom config: Browse for a .toml file");
    ui.label("• Edit mappings: Load a config, modify, then save");

    ui.add_space(10.0);
    ui.label("Backend CLI tool: config-2014-naga");
    ui.label("Install with: cargo install config-2014-naga");
}
