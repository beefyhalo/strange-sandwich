use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::{ChatMessage, StructuredOutputFormat},
    LLMProvider,
};
use schemars::schema_for;
use strange_sandwich_core::Recipe;


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
