#[derive(Default)]
pub struct AppState {
    pub remapping_enabled: bool,
}

#[derive(PartialEq)]
pub enum Tab {
    KeyMappings,
    Settings,
    About,
}
