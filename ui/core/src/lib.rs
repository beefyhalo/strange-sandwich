use schemars::JsonSchema;

use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug, Clone)]
pub struct Recipe {
    pub title: String,
    pub description: String,
    pub author: Option<String>,

    pub prep_time: Option<String>,    // ISO 8601 (e.g. PT30M)
    pub cook_time: Option<String>,    // ISO 8601
    pub total_time: Option<String>,   // ISO 8601
    pub yield_amount: Option<String>, // e.g. "4 servings"

    pub article: Option<String>,
    pub ingredients: Vec<String>,
    pub steps: Vec<RecipeStep>,

    pub image: Option<String>, // URL
    pub tags: Option<Vec<String>>,
    pub nutrition: Option<NutritionInfo>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RecipeStep {
    pub description: String,
    pub image: Option<String>,    // URL
    pub duration: Option<String>, // ISO 8601 (e.g. PT10M)
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NutritionInfo {
    pub calories: Option<String>,
    pub fat_content: Option<String>,
    pub carbohydrate_content: Option<String>,
    pub protein_content: Option<String>,
    pub fiber_content: Option<String>,
    pub sugar_content: Option<String>,
}
