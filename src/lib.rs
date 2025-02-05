pub mod cs_gui;
pub mod services;

pub fn connect_to_database(_connection_string: &str) {}

pub fn run_gui() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "PNT CS Tool",
        native_options,
        Box::new(|cc| Ok(Box::new(cs_gui::app::MyEguiApp::new(cc)))),
    );
}
