use glider_kitchen_ai::TypeOfIngredient;

fn main() {
    let mut ai = glider_kitchen_ai::KitchenAi::new(String::from("config.toml"));

    ai.add_ingredient(TypeOfIngredient::FRUIT, &String::from("banana"))
        .expect("Should work");
    println!("Ratio fruits: {}", ai.get_ratio(TypeOfIngredient::FRUIT));
    println!(
        "Ratio vegetables: {}",
        ai.get_ratio(TypeOfIngredient::VEGETABLE)
    );

    ai.add_ingredient(TypeOfIngredient::FRUIT, &String::from("peach"))
        .expect("Should work");
    println!("Ratio fruits: {}", ai.get_ratio(TypeOfIngredient::FRUIT));

    println!(
        "Ratio fruits is valid: {}",
        ai.is_valid(TypeOfIngredient::FRUIT)
    );
}
