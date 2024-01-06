use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Error;
use std::fs;

#[derive(Eq, PartialEq, Hash)]
pub enum TypeOfIngredient {
    FRUITS,
    VEGETABLES,
}
type Ratio = f32;
type Table = HashMap<String, Ratio>;
#[derive(Deserialize, Debug)]
struct RatioTable {
    fruits: HashMap<String, Ratio>,
    vegetables: HashMap<String, Ratio>,
}

#[derive(Default)]
struct Recipe {
    ratio: Ratio,
    ingredients: Vec<String>,
}

impl Recipe {
    fn add_ingredient_to_recipe(
        &mut self,
        ingredient: &String,
        table: &Table,
    ) -> Result<(), Error> {
        if !self.ingredients.contains(ingredient) {
            self.ingredients.push(ingredient.clone());
            self.ratio = self.compute_ratio(table);
            Ok(())
        } else {
            Err(Default::default())
        }
    }

    fn compute_ratio(&self, ratio_table: &Table) -> Ratio {
        let ingredients = &self.ingredients;
        let t: Ratio = ingredients
            .iter()
            .map(|i| {
                ratio_table
                    .get(i.as_str())
                    .expect("Ingredient should exist in the table")
            })
            .sum();
        t / self.ingredients.len() as Ratio
    }
}

pub struct KitchenAi {
    ratio_tables: HashMap<TypeOfIngredient, Table>,
    recipes: HashMap<TypeOfIngredient, Recipe>,
}

fn load_config(config_file_path: String) -> RatioTable {
    let content = fs::read_to_string(config_file_path).expect("Config file needs to be readable");
    toml::from_str(content.as_str()).unwrap()
}

impl KitchenAi {
    pub fn new(config_file_path: String) -> KitchenAi {
        let ratio_table = load_config(config_file_path);

        println!("Vegetables:");
        let vegetables = &ratio_table.vegetables;
        for (key, value) in vegetables.into_iter() {
            println!("{} = {}", key, value);
        }
        println!("Fruits:");
        let fruits = &ratio_table.fruits;
        for (key, value) in fruits.into_iter() {
            println!("{} = {}", key, value);
        }
        KitchenAi {
            ratio_tables: HashMap::from([
                (TypeOfIngredient::FRUITS, ratio_table.fruits),
                (TypeOfIngredient::VEGETABLES, ratio_table.vegetables),
            ]),

            recipes: Default::default(),
        }
    }

    pub fn add_ingredient(
        &mut self,
        type_of_ingredient: TypeOfIngredient,
        ingredient: &String,
    ) -> Result<(), Error> {
        match self.ratio_tables.get(&type_of_ingredient) {
            None => Err(Default::default()),
            Some(table) => {
                self.recipes
                    .entry(type_of_ingredient)
                    .or_default()
                    .add_ingredient_to_recipe(ingredient, table)
                    .expect("Ingredient should have been added");
                Ok(())
            }
        }
    }

    pub fn get_ratio(&self, type_of_ingredient: TypeOfIngredient) -> Ratio {
        match self.recipes.get(&type_of_ingredient) {
            None => Default::default(),
            Some(recipe) => recipe.ratio,
        }
    }
}
