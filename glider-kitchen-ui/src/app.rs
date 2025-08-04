use egui::Ui;
use glider_kitchen_ai::{KitchenAi, TypeOfIngredient};
use log::info;
use std::collections::{HashMap, HashSet};
use std::default::Default;

pub struct KitchenApp {
    ai: KitchenAi,
    tables_loaded: bool,
    available_ingredients: HashMap<TypeOfIngredient, HashSet<String>>,
}

impl KitchenApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut ai = KitchenAi::new_with_config_content(include_str!("../../config.toml"));
        ai.load_tables_from_content(include_str!("../../table.toml"));

        KitchenApp {
            ai,
            tables_loaded: true,
            available_ingredients: Default::default(),
        }
    }

    fn display_ingredients(&mut self, ui: &mut Ui, type_of_ingredient: &TypeOfIngredient) {
        let ingredients = self
            .ai
            .get_ingredients_from_table(type_of_ingredient)
            .expect("List of ingredients");
        let selected_ingredients = match self.ai.get_recipe(type_of_ingredient) {
            Ok(recipe) => recipe.get_ingredients().clone(),
            _ => {
                info!("No recipe yet for {type_of_ingredient}!");
                HashSet::new()
            }
        };

        // Copy vector as we need to release self.ai, and sort it so display is stable
        let mut sorted_ingredients: Vec<String> =
            ingredients.into_iter().map(|s| s.to_string()).collect();
        sorted_ingredients.sort();

        egui::Grid::new(type_of_ingredient).show(ui, |ui| {
            let mut idx = 0;
            for ingredient in sorted_ingredients {
                let mut label = ingredient.to_owned();
                let ingredient_selected = selected_ingredients.contains(&label);
                if ingredient_selected {
                    label += "*";
                }

                let ingredient_enabled = if !selected_ingredients.is_empty() {
                    self.available_ingredients
                        .get(type_of_ingredient)
                        .unwrap_or(&HashSet::new())
                        .contains(&ingredient)
                } else {
                    true
                };

                if ui
                    .add_enabled(ingredient_enabled, egui::Button::new(label))
                    .clicked()
                {
                    if ingredient_selected {
                        self.ai
                            .remove_ingredient(type_of_ingredient, &ingredient)
                            .expect("All good!");
                    } else {
                        self.ai
                            .add_ingredient(type_of_ingredient, &ingredient)
                            .expect("All good!");
                    }
                    self.recompute_predictions(type_of_ingredient);
                }

                idx += 1;
                if idx % 5 == 0 {
                    ui.end_row();
                }
            }
        });
    }

    fn recompute_predictions(&mut self, type_of_ingredient: &TypeOfIngredient) {
        let predictions = self.ai.predict(type_of_ingredient).expect("All good!");
        // println!("Predictions: {predictions:?}");

        // Concatenate all ingredients from all recipes into a single HashSet
        let mut all_ingredients = HashSet::new();
        for recipe in predictions {
            all_ingredients.extend(recipe.get_ingredients().iter().cloned());
        }

        self.available_ingredients
            .insert(*type_of_ingredient, all_ingredients);
    }

    fn display_current_ratio(&self, ui: &mut Ui, type_of_ingredient: &TypeOfIngredient) {
        let ratio = self.ai.get_ratio(type_of_ingredient);
        let ratio_valid = self.ai.is_valid(type_of_ingredient);
        let color = if ratio_valid {
            egui::Color32::GREEN
        } else {
            egui::Color32::RED
        };
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new(ratio.to_string())
                    .size(20.0)
                    .color(color),
            );
        });
    }
}

impl eframe::App for KitchenApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel is the region left after adding TopPanel's and SidePanel's

            ui.heading("Glider Kitchen");
            ui.hyperlink("https://github.com/audricschiltknecht/glider-kitchen");
            egui::warn_if_debug_build(ui);

            ui.separator();

            if self.tables_loaded {
                ui.horizontal(|ui| {
                    self.display_ingredients(ui, &TypeOfIngredient::VEGETABLE);
                    ui.separator();
                    self.display_current_ratio(ui, &TypeOfIngredient::VEGETABLE);
                });

                ui.separator();

                ui.horizontal(|ui| {
                    self.display_ingredients(ui, &TypeOfIngredient::FRUIT);
                    ui.separator();
                    self.display_current_ratio(ui, &TypeOfIngredient::FRUIT);
                });
            }
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
