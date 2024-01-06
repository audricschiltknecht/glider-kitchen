mod app;
use crate::app::KitchenApp;
use glider_kitchen_ai::TypeOfIngredient;

fn main() {
    let mut ai = glider_kitchen_ai::KitchenAi::new("config.toml", "table.toml");

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

}
