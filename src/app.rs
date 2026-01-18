use eframe::egui;
use std::fs;
use std::path::PathBuf;
use std::sync::{
    Arc,
    Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

use crate::tabs;
use crate::state::{AppState, Tab};
use config_2014_naga::{key_map::KeyMapper, run_loop};

pub struct ConfigNagaGuiApp {
    pub state: Arc<Mutex<AppState>>,
    pub key_mappings: Vec<(u8, String)>,
    pub backend_thread: Option<thread::JoinHandle<()>>,
    pub running: Arc<AtomicBool>,
    pub current_tab: Tab,
    pub status_message: String,
    pub config_dir: PathBuf,
    pub loaded_config_path: Option<PathBuf>,
    pub valid_keys: Vec<String>,
    pub key_mapper: KeyMapper,
}

impl ConfigNagaGuiApp {
    pub fn new(state: Arc<Mutex<AppState>>) -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("config-2014-naga");

        let _ = fs::create_dir_all(&config_dir);

        let loaded_config_path = Self::load_last_config_path(&config_dir);

        let mut app = Self {
            state,
            key_mappings: Vec::new(),
            backend_thread: None,
            running: Arc::new(AtomicBool::new(false)),
            current_tab: Tab::KeyMappings,
            status_message: "Ready - No config loaded (will use default)".to_string(),
            config_dir,
            loaded_config_path: None,
            valid_keys: Self::get_valid_keys(),
            key_mapper: KeyMapper::default(),
        };

        if let Some(path) = loaded_config_path {
            if path.exists() {
                app.load_config_from_path(&path);
            }
        }

        app
    }

    fn get_valid_keys() -> Vec<String> {
        vec![
            "F1","F2","F3","F4","F5","F6","F7","F8","F9","F10","F11","F12",
            "Grave","_1","_2","_3","_4","_5","_6","_7","_8","_9","_0","Minus","Equal","BackSpace",
            "Tab","Q","W","E","R","T","Y","U","I","O","P","LeftBrace","RightBrace","BackSlash",
            "CapsLock","A","S","D","F","G","H","J","K","L","SemiColon","Apostrophe","Enter",
            "LeftShift","Z","X","C","V","B","N","M","Comma","Dot","Slash","RightShift",
            "LeftControl","LeftMeta","LeftAlt","Space","RightAlt","RightMeta","RightControl",
            "Esc","SysRq","ScrollLock","Insert","Home","PageUp","Delete","End","PageDown",
            "Up","Left","Down","Right","NumLock","LineFeed","ScrollUp","ScrollDown",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    pub fn browse_for_config(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("TOML Config", &["toml"])
            .pick_file()
        {
            self.load_config_from_path(&path);
        }
    }

    pub fn save_config(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("TOML Config", &["toml"])
            .set_directory(&self.config_dir)
            .save_file()
        {
            let content = self.generate_toml_config();
            match fs::write(&path, content) {
                Ok(_) => {
                    self.status_message = format!("Config saved to {}", path.display());
                    self.save_last_config_path(&path);
                    self.loaded_config_path = Some(path.clone());
                    self.key_mapper = KeyMapper::read_from_file(path.to_str().unwrap())
                        .unwrap_or_default();
                }
                Err(e) => self.status_message = format!("Error saving config: {e}"),
            }
        }
    }

    fn load_config_from_path(&mut self, path: &PathBuf) {
        match fs::read_to_string(path) {
            Ok(content) => {
                if let Ok(parsed) = toml::from_str::<toml::Value>(&content) {
                    if let Some(keys) = parsed.get("keys").and_then(|v| v.as_table()) {
                        self.key_mappings.clear();
                        for i in 1..=12 {
                            if let Some(v) = keys.get(&i.to_string()).and_then(|v| v.as_str()) {
                                self.key_mappings.push((i, v.to_string()));
                            }
                        }
                        self.key_mapper = KeyMapper::read_from_file(path.to_str().unwrap())
                            .unwrap_or_default();
                        self.status_message = format!("Loaded config from {}", path.display());
                        self.save_last_config_path(path);
                        self.loaded_config_path = Some(path.clone());
                        return;
                    }
                }
                self.status_message = "Error parsing config file".to_string();
            }
            Err(e) => self.status_message = format!("Error loading config: {e}"),
        }
    }

    fn generate_toml_config(&self) -> String {
        let mut out = String::from("[keys]\n");
        for (num, key) in &self.key_mappings {
            out.push_str(&format!("{num} = '{key}'\n"));
        }
        out
    }

    pub fn is_remapping_active(&self) -> bool {
        self.backend_thread.is_some()
    }

    pub fn start_remapping(&mut self) {
        if self.backend_thread.is_some() {
            self.status_message = "Already running - click Stop first".to_string();
            return;
        }

        self.running = Arc::new(AtomicBool::new(true));
        let running_clone = self.running.clone();
        let mapper_clone = self.key_mapper.clone();

        let handle = thread::spawn(move || {
            if let Err(e) = run_loop(mapper_clone, running_clone) {
                eprintln!("Remapping backend error: {}", e);
            }
        });

        self.backend_thread = Some(handle);
        self.status_message = "Remapping started".to_string();

        if let Ok(mut s) = self.state.lock() {
            s.remapping_enabled = true;
        }
    }

    pub fn stop_remapping(&mut self) {
        if self.backend_thread.is_none() {
            self.status_message = "Not running".to_string();
            return;
        }

        self.running.store(false, Ordering::SeqCst);
        self.backend_thread = None;

        self.status_message =
            "Stopping... (press any Naga button to complete)".to_string();

        if let Ok(mut s) = self.state.lock() {
            s.remapping_enabled = false;
        }
    }

    fn save_last_config_path(&self, path: &PathBuf) {
        let file = self.config_dir.join("last_config.txt");
        let _ = fs::write(file, path.to_string_lossy().as_bytes());
    }

    fn load_last_config_path(config_dir: &PathBuf) -> Option<PathBuf> {
        fs::read_to_string(config_dir.join("last_config.txt"))
            .ok()
            .map(|s| PathBuf::from(s.trim()))
    }
}


impl eframe::App for ConfigNagaGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📂 Browse for Config...").clicked() {
                        self.browse_for_config();
                        ui.close_menu();
                    }
                    if ui.button("💾 Save Config As...").clicked() {
                        self.save_config();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("❌ Quit").clicked() {
                        self.stop_remapping();
                        std::process::exit(0);
                    }
                });

                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if self.is_remapping_active() {
                            ui.colored_label(
                                egui::Color32::GREEN,
                                "● ACTIVE",
                            );
                        } else {
                            ui.colored_label(
                                egui::Color32::GRAY,
                                "● INACTIVE",
                            );
                        }
                    },
                );
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Navigation");
            ui.separator();
            ui.selectable_value(
                &mut self.current_tab,
                Tab::KeyMappings,
                "🎮 Key Mappings",
            );
            ui.selectable_value(
                &mut self.current_tab,
                Tab::Settings,
                "⚙ Settings",
            );
            ui.selectable_value(
                &mut self.current_tab,
                Tab::About,
                "ℹ About",
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::KeyMappings => tabs::key_mappings::show(self, ui),
                Tab::Settings => tabs::settings::show(self, ui),
                Tab::About => tabs::about::show(self, ui),
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Runs only on actual process exit
        self.stop_remapping();
    }
}
