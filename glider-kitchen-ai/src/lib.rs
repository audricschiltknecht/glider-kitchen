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
    min_ingredients: u8,
    max_ingredients: u8,
}

#[derive(Default, Clone, Debug)]
pub struct Recipe {
    ratio: Ratio,
    ingredients: Vec<String>,
}

impl Recipe {
    fn add_ingredient_to_recipe(
        &mut self,
        ingredient: &str,
        table: &IngredientToRatio,
    ) -> Result<(), Error> {
        let owned_ingredient = ingredient.to_string();
        if !self.ingredients.contains(&owned_ingredient) {
            self.ingredients.push(owned_ingredient);
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

fn load_file<T>(config_file_path: &str) -> T
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
    pub fn new(config_filepath: &str, ratio_tables_filepath: &str) -> KitchenAi {
        let config: Configuration = load_file(config_filepath);
        let ratio_table: RatioTable = load_file(ratio_tables_filepath);

        let table_per_type = ratio_table_to_table_per_type(ratio_table);

        println!("Vegetables:");
        let vegetables = &table_per_type[&TypeOfIngredient::VEGETABLE];
        for (key, value) in vegetables.iter() {
            println!("{} = {}", key, value);
        }
        println!("Fruits:");
        let fruits = &table_per_type[&TypeOfIngredient::FRUIT];
        for (key, value) in fruits.iter() {
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
        ingredient: &str,
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
        self.config.min_ratio <= ratio && ratio <= self.config.max_ratio
    }

    pub fn predict(&self, type_of_ingredient: TypeOfIngredient) -> Result<Vec<Recipe>, Error> {
        match self.ratio_tables.get(&type_of_ingredient) {
            None => Err(Default::default()),
            Some(ratios) => {
                let recipe = match self.recipes.get(&type_of_ingredient) {
                    None => Default::default(),
                    Some(r) => r.clone(),
                };
                match self._internal_predict(&recipe, ratios) {
                    None => Err(Default::default()),
                    Some(recipes) => Ok(recipes),
                }
            }
        }
    }

    fn _internal_predict(
        &self,
        recipe: &Recipe,
        ratios: &IngredientToRatio,
    ) -> Option<Vec<Recipe>> {
        let number_ingredients = recipe.ingredients.len();

        if number_ingredients == self.config.max_ingredients as usize {
            return if self.config.min_ratio <= recipe.ratio && recipe.ratio <= self.config.max_ratio
            {
                Some(vec![recipe.clone()])
            } else {
                None
            };
        }

        let mut out: Vec<Recipe> = Default::default();
        if number_ingredients >= self.config.min_ingredients as usize
            && self.config.min_ratio <= recipe.ratio
            && recipe.ratio <= self.config.max_ratio
        {
            out.push(recipe.clone());
        }

        let next_ingredients = ratios.keys();
        for ingredient in next_ingredients {
            let mut new_recipe = recipe.clone();
            match new_recipe.add_ingredient_to_recipe(ingredient, ratios) {
                Err(_) => {} // Ingredient already in recipe
                Ok(()) => match self._internal_predict(&new_recipe, ratios) {
                    None => {}
                    Some(possibilities) => out.extend(possibilities),
                },
            }
        }
        Some(out)
    }
}
