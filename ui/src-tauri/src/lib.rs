use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::{ChatMessage, StructuredOutputFormat},
    LLMProvider,
};

use schemars::{schema_for, JsonSchema};

use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Default, Debug)]
pub struct Recipe {
    pub title: String,
    pub description: Option<String>,
    pub author: Option<String>,

    pub prep_time: Option<String>,    // ISO 8601 (e.g. PT30M)
    pub cook_time: Option<String>,    // ISO 8601
    pub total_time: Option<String>,   // ISO 8601
    pub yield_amount: Option<String>, // e.g. "4 servings"

    pub ingredients: Vec<String>,
    pub steps: Vec<RecipeStep>,

    pub image: Option<String>, // URL
    pub tags: Option<Vec<String>>,
    pub nutrition: Option<NutritionInfo>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct RecipeStep {
    pub description: String,
    pub image: Option<String>,    // URL
    pub duration: Option<String>, // ISO 8601 (e.g. PT10M)
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct NutritionInfo {
    pub calories: Option<String>,
    pub fat_content: Option<String>,
    pub carbohydrate_content: Option<String>,
    pub protein_content: Option<String>,
    pub fiber_content: Option<String>,
    pub sugar_content: Option<String>,
}

#[tauri::command]
async fn greet(name: &str, llm: tauri::State<'_, Box<dyn LLMProvider>>) -> Result<Recipe, String> {
    let prompt = format!(
        "Generate a creative sandwich recipe using {} in JSON format.",
        name
    );

    let messages = vec![ChatMessage::user().content(prompt).build()];

    match llm.chat(&messages).await {
        Ok(text) => {
            let recipe: Recipe = serde_json::from_str(text.text().unwrap().as_str())
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;
            println!("Generated recipe: {:?}", recipe);
            Ok(recipe)
        },
        Err(e) => Err(format!("Chat error: {}", e)),
    }
}

pub fn run() {
    let schema = {
        let schema = serde_json::to_value(schema_for!(Recipe)).expect("Failed to serialize schema");
        StructuredOutputFormat {
            name: "Recipe".to_string(),
            description: Some("A creative sandwich recipe".to_string()),
            schema: Some(schema),
            strict: None,
        }
    };

    let llm = LLMBuilder::new()
        .backend(LLMBackend::Ollama)
        .model("smollm2:latest")
        .max_tokens(1000)
        // .temperature(0.7)
        .stream(false)
        .schema(schema)
        .system("You are a helpful AI assistant. Please generate strange sandwich recipes using the provided JSON schema. 
                 Make the recipe as strange as creative as possible. The ingredients should be unusual and the steps should be unique.
                 The tone should be humorous and light-hearted. Don't worry about the practicality of the recipe. Focus on making it fun and entertaining.
                 Ensure that the recipe is in JSON format and follows the provided schema.
                 Ensure that the user's input is included in the recipe.")
        .build()
        .expect("Failed to build LLM (Ollama)");

    tauri::Builder::default()
        .manage(llm)
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
