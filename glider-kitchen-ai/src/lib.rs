use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::fs;

#[derive(Deserialize, Eq, PartialEq, Hash, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TypeOfIngredient {
    FRUIT,
    VEGETABLE,
}

impl Display for TypeOfIngredient {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            TypeOfIngredient::FRUIT => write!(f, "FRUIT"),
            TypeOfIngredient::VEGETABLE => write!(f, "VEGETABLE"),
        }
    }
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
pub struct Configuration {
    min_ratio: Ratio,
    max_ratio: Ratio,
    min_ingredients: u8,
    max_ingredients: u8,
}

#[derive(Default, Clone, Debug)]
pub struct Recipe {
    ratio: Ratio,
    ingredients: HashSet<String>,
}
impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.ingredients == other.ingredients
    }
}
impl Eq for Recipe {}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for ingredient in &self.ingredients {
            ingredient.hash(state);
        }
    }
}

impl Recipe {
    fn add_ingredient_to_recipe(
        &mut self,
        ingredient: &str,
        table: &IngredientToRatio,
    ) -> Result<(), Error> {
        let owned_ingredient = ingredient.to_string();
        if !self.ingredients.contains(&owned_ingredient) {
            self.ingredients.insert(owned_ingredient);
            self.ratio = self.compute_ratio(table);
            Ok(())
        } else {
            Err(Default::default())
        }
    }

    fn remove_ingredient_from_recipe(
        &mut self,
        ingredient: &str,
        table: &IngredientToRatio,
    ) -> Result<(), Error> {
        if self.ingredients.contains(ingredient) {
            self.ingredients.remove(ingredient);
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

    pub fn get_ingredients(&self) -> &HashSet<String> {
        &self.ingredients
    }
}

pub struct KitchenAi {
    config: Configuration,
    ratio_tables: HashMap<TypeOfIngredient, IngredientToRatio>,
    recipes: HashMap<TypeOfIngredient, Recipe>,
}

fn load_from_file<T>(path: &Path) -> T
where
    T: DeserializeOwned,
{
    let content = fs::read_to_string(path).unwrap_or_else(|_| { panic!("Config file {} needs to be readable", path.to_str().unwrap()) });
    toml::from_str(content.as_str()).unwrap()
}

pub fn load_from_content<T>(content: &str) -> T
where
    T: DeserializeOwned,
{
    toml::from_str(content).unwrap()
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
    pub fn new(config_filepath: &Path) -> KitchenAi {
        let config: Configuration = load_from_file(config_filepath);

        KitchenAi {
            config,
            ratio_tables: Default::default(),
            recipes: Default::default(),
        }
    }

    pub fn new_with_config_content(config_content: &str) -> KitchenAi {
        let config: Configuration = load_from_content(config_content);
        KitchenAi {
            config,
            ratio_tables: Default::default(),
            recipes: Default::default(),
        }
    }

    pub fn new_with_config(config: Configuration) -> KitchenAi {
        KitchenAi {
            config,
            ratio_tables: Default::default(),
            recipes: Default::default(),
        }
    }

    pub fn load_tables_from_file(&mut self, ratio_tables_filepath: &Path) {
        let ratio_table: RatioTable = load_from_file(ratio_tables_filepath);
        self.ratio_tables = ratio_table_to_table_per_type(ratio_table);
    }

    pub fn load_tables_from_content(&mut self, ratio_tables_content: &str) {
        let ratio_table: RatioTable = load_from_content(ratio_tables_content);
        self.ratio_tables = ratio_table_to_table_per_type(ratio_table);
    }

    pub fn get_ingredients_from_table(
        &self,
        type_of_ingredient: &TypeOfIngredient,
    ) -> Result<Vec<&str>, Error> {
        match self.ratio_tables.get(type_of_ingredient) {
            None => Err(Default::default()),
            Some(table) => {
                Ok(table.keys().map(|s| s.as_str()).collect())
                // let mut out = HashSet::new();
                // out.extend(table.keys());
                // Ok(out)
            }
        }
    }

    pub fn get_recipe(&self, type_of_ingredient: &TypeOfIngredient) -> Result<&Recipe, Error> {
        match self.recipes.get(type_of_ingredient) {
            None => Err(Default::default()),
            Some(recipe) => Ok(recipe),
        }
    }

    pub fn add_ingredient(
        &mut self,
        type_of_ingredient: &TypeOfIngredient,
        ingredient: &str,
    ) -> Result<(), Error> {
        match self.ratio_tables.get(type_of_ingredient) {
            None => Err(Default::default()),
            Some(table) => {
                self.recipes
                    .entry(*type_of_ingredient)
                    .or_default()
                    .add_ingredient_to_recipe(ingredient, table)
                    .expect("Ingredient should have been added");
                Ok(())
            }
        }
    }

    pub fn remove_ingredient(
        &mut self,
        type_of_ingredient: &TypeOfIngredient,
        ingredient: &str,
    ) -> Result<(), Error> {
        match self.ratio_tables.get(type_of_ingredient) {
            None => Err(Default::default()),
            Some(table) => {
                self.recipes
                    .entry(*type_of_ingredient)
                    .or_default()
                    .remove_ingredient_from_recipe(ingredient, table)
                    .expect("Ingredient should have been removed");
                Ok(())
            }
        }
    }

    pub fn get_ratio(&self, type_of_ingredient: &TypeOfIngredient) -> Ratio {
        match self.recipes.get(type_of_ingredient) {
            None => Default::default(),
            Some(recipe) => recipe.ratio,
        }
    }

    pub fn is_valid(&self, type_of_ingredient: &TypeOfIngredient) -> bool {
        let ratio = self.get_ratio(type_of_ingredient);
        self.config.min_ratio <= ratio && ratio <= self.config.max_ratio
    }

    pub fn predict(&self, type_of_ingredient: &TypeOfIngredient) -> Result<HashSet<Recipe>, Error> {
        match self.ratio_tables.get(type_of_ingredient) {
            None => Err(Default::default()),
            Some(ratios) => {
                let recipe = match self.recipes.get(type_of_ingredient) {
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
    ) -> Option<HashSet<Recipe>> {
        let number_ingredients = recipe.ingredients.len();

        if number_ingredients == self.config.max_ingredients as usize {
            return if self.config.min_ratio <= recipe.ratio && recipe.ratio <= self.config.max_ratio
            {
                let mut out = HashSet::new();
                out.insert(recipe.clone());
                Some(out)
            } else {
                None
            };
        }

        let mut out: HashSet<Recipe> = Default::default();
        if number_ingredients >= self.config.min_ingredients as usize
            && self.config.min_ratio <= recipe.ratio
            && recipe.ratio <= self.config.max_ratio
        {
            out.insert(recipe.clone());
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
