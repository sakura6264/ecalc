#![windows_subsystem = "windows"]
mod tools;
fn main() {
    let ico = image::load_from_memory(include_bytes!("../assets/icon.png")).unwrap().to_rgba8();
    let option = eframe::NativeOptions{
        initial_window_size: Some(eframe::egui::Vec2::new(1000.0, 650.0)),
        icon_data: Some(eframe::IconData{
            rgba: ico.into_raw(),
            width:64,
            height: 64,
        }),
        ..Default::default()
    };
    eframe::run_native("ECalc",option,Box::new(|_cc|Box::new(tools::gui::App::new()))).unwrap();
}
