#[derive(Default)]
pub struct AppState {
    //pub window_visible: bool,
    pub remapping_enabled: bool,
}

#[derive(PartialEq)]
pub enum Tab {
    KeyMappings,
    Settings,
    About,
}
