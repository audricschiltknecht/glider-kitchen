mod app;

use crate::app::KitchenApp;
use std::path::Path;

use glider_kitchen_ai::TypeOfIngredient;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut ai = glider_kitchen_ai::KitchenAi::new(Path::new("./config.toml"));
    ai.load_tables(Path::new("./table.toml"));

    ai.add_ingredient(TypeOfIngredient::FRUIT, "pomme avec pelure")
        .expect("Should work");
            ai.add_ingredient(TypeOfIngredient::FRUIT, "banane")
                .expect("Should work");
    println!("Ration: {:?}", ai.get_ratio(TypeOfIngredient::FRUIT));
    let predictions = ai.predict(TypeOfIngredient::FRUIT).expect("Should work");

    println!("Predictions: {:?}", predictions);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Glider Kitchen",
        native_options,
        Box::new(|cc| Box::new(KitchenApp::new(cc))),
    )
}
