use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Error;
use std::fs;

#[derive(Deserialize, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TypeOfIngredient {
    FRUIT,
    VEGETABLE,
}
type Ratio = f32;
type IngredientToRatio = HashMap<String, Ratio>;

#[derive(Deserialize, Debug)]
struct RatioEntry {
    name: String,
    #[serde(rename = "type")]
    type_of_ingredient: TypeOfIngredient,
    ratio: Ratio,
}

#[derive(Deserialize, Debug)]
struct RatioTable {
    entry: Vec<RatioEntry>,
}

#[derive(Deserialize, Debug)]
struct Configuration {
    min_ratio: Ratio,
    max_ratio: Ratio,
    table_file_path: String,
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
        table: &IngredientToRatio,
    ) -> Result<(), Error> {
        if !self.ingredients.contains(ingredient) {
            self.ingredients.push(ingredient.clone());
            self.ratio = self.compute_ratio(table);
            Ok(())
        } else {
            Err(Default::default())
        }
    }

    fn compute_ratio(&self, ratio_table: &IngredientToRatio) -> Ratio {
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
    config: Configuration,
    ratio_tables: HashMap<TypeOfIngredient, IngredientToRatio>,
    recipes: HashMap<TypeOfIngredient, Recipe>,
}

fn load_file<T>(config_file_path: &String) -> T
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(config_file_path).expect("Config file needs to be readable");
    toml::from_str(content.as_str()).unwrap()
}

fn ratio_table_to_table_per_type(
    tables: RatioTable,
) -> HashMap<TypeOfIngredient, IngredientToRatio> {
    let mut out: HashMap<TypeOfIngredient, IngredientToRatio> = Default::default();
    for entry in tables.entry {
        out.entry(entry.type_of_ingredient)
            .or_default()
            .insert(entry.name, entry.ratio);
    }
    out
}

impl KitchenAi {
    pub fn new(config_file_path: String) -> KitchenAi {
        let config: Configuration = load_file(&config_file_path);
        let ratio_table: RatioTable = load_file(&config.table_file_path);

        let table_per_type = ratio_table_to_table_per_type(ratio_table);

        println!("Vegetables:");
        let vegetables = &table_per_type[&TypeOfIngredient::VEGETABLE];
        for (key, value) in vegetables.into_iter() {
            println!("{} = {}", key, value);
        }
        println!("Fruits:");
        let fruits = &table_per_type[&TypeOfIngredient::FRUIT];
        for (key, value) in fruits.into_iter() {
            println!("{} = {}", key, value);
        }
        KitchenAi {
            config,
            ratio_tables: table_per_type,
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

    pub fn is_valid(&self, type_of_ingredient: TypeOfIngredient) -> bool {
        let ratio = self.get_ratio(type_of_ingredient);
        return self.config.min_ratio <= ratio && ratio <= self.config.max_ratio;
    }
}
