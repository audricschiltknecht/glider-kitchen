mod app;

use crate::app::KitchenApp;
use std::path::Path;

use glider_kitchen_ai::TypeOfIngredient;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut ai = glider_kitchen_ai::KitchenAi::new(Path::new("./config.toml"));
    ai.load_tables(Path::new("./table.toml"));

    ai.add_ingredient(TypeOfIngredient::FRUIT, "banana")
        .expect("Should work");
    println!("Ratio fruits: {}", ai.get_ratio(TypeOfIngredient::FRUIT));
    println!(
        "Ratio vegetables: {}",
        ai.get_ratio(TypeOfIngredient::VEGETABLE)
    );

    ai.add_ingredient(TypeOfIngredient::FRUIT, "peach")
        .expect("Should work");
    println!("Ratio fruits: {}", ai.get_ratio(TypeOfIngredient::FRUIT));

    println!(
        "Ratio fruits is valid: {}",
        ai.is_valid(TypeOfIngredient::FRUIT)
    );

    let predictions = ai.predict(TypeOfIngredient::FRUIT).expect("Should work");

    println!("Predictions: {:?}", predictions);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Glider Kitchen",
        native_options,
        Box::new(|cc| Box::new(KitchenApp::new(cc))),
    )
}
