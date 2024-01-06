use glider_kitchen_ai::TypeOfIngredient;

fn main() {
    let mut ai = glider_kitchen_ai::KitchenAi::new(String::from("table.toml"));
    ai.add_ingredient(TypeOfIngredient::FRUITS, &String::from("banane"))
        .expect("Should work");
    println!("Ratio fruits: {}", ai.get_ratio(TypeOfIngredient::FRUITS));
    ai.add_ingredient(TypeOfIngredient::FRUITS, &String::from("peche"))
        .expect("Should work");
    println!("Ratio fruits: {}", ai.get_ratio(TypeOfIngredient::FRUITS));
}
