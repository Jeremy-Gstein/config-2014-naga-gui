use std::sync::{Arc, Mutex};
use crate::state::AppState;

pub fn setup_system_tray(app_state: Arc<Mutex<AppState>>) {
    use ksni::{MenuItem, menu::StandardItem, Tray, TrayService};

    struct NagaTray {
        state: Arc<Mutex<AppState>>,
    }

    impl Tray for NagaTray {
        fn title(&self) -> String {
            "Config 2014 Naga GUI".into()
        }

        fn icon_name(&self) -> String {
            let active = self.state
                .lock()
                .map(|s| s.remapping_enabled)
                .unwrap_or(false);

            if active {
                "input-gaming".into()  // System icon - gaming controller
            } else {
                "input-mouse".into()   // System icon - mouse
            }
        }

        fn menu(&self) -> Vec<MenuItem<Self>> {
            let is_enabled = self.state
                .lock()
                .map(|s| s.remapping_enabled)
                .unwrap_or(false);

            vec![
                MenuItem::Standard(StandardItem {
                    label: if is_enabled {
                        "● Remapping Active".into()
                    } else {
                        "○ Remapping Inactive".into()
                    },
                    enabled: false,
                    ..Default::default()
                }),
                MenuItem::Separator,
                MenuItem::Standard(StandardItem {
                    label: "Quit".into(),
                    activate: Box::new(|_| std::process::exit(0)),
                    ..Default::default()
                }),
            ]
        }
    }

    let service = TrayService::new(NagaTray { state: app_state.clone() });
    let handle = service.handle();

    std::thread::spawn(move || {
        let mut last = false;

        loop {
            let current = app_state
                .lock()
                .map(|s| s.remapping_enabled)
                .unwrap_or(false);

            if current != last {
                let _ = handle.update(|_tray| {}); // refresh menu and icon
                last = current;
            }

            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    });

    let _ = service.run();
}
