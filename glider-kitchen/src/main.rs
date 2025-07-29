mod app;

use crate::app::KitchenApp;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Glider Kitchen",
        native_options,
        Box::new(|cc| Ok(Box::new(KitchenApp::new(cc)))),
    )
}
