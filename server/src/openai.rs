use serde_json::json;

#[derive(Clone)]
pub struct Client {
    pub api_key: String,
    pub client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Client {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_request(&self, prompt: &str) -> Result<String, reqwest::Error> {
        let response = self
            .client
            .post("https://api.openai.com/v1/completions")
            .bearer_auth(&self.api_key)
            .json(&json!({"model": "o4-mini","prompt": prompt,"max_tokens": 300}))
            .send()
            .await?;

        response.text().await
    }
}
