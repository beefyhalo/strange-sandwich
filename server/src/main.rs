#[macro_use]
extern crate dotenv_codegen;

mod openai;

#[tokio::main]
async fn main() {
    let api_key = dotenv!("OPENAI_API_KEY").to_owned();
    let api_client = openai::Client::new(api_key);
    let routes = filters::sandwich(api_client);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    use crate::{handlers, openai};
    use warp::Filter;

    pub fn sandwich(
        api_client: openai::Client,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path("generate")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || api_client.clone()))
            .and_then(handlers::generate_sandwich)
    }
}

mod handlers {
    use super::models::Ingredients;
    use crate::openai;
    use serde_json::json;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn generate_sandwich(
        ingredients: Ingredients,
        client: openai::Client,
    ) -> Result<impl warp::Reply, Infallible> {
        let prompt = format!(
            "Generate a creative sandwich recipe using {} in JSON format.",
            ingredients.ingredients
        );

        let response = client.send_request(&prompt).await;

        let (body, status) = match response {
            Ok(body) => (json!({ "result": body }), StatusCode::OK),
            Err(_) => (
                json!({ "error": "API request failed" }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        Ok(warp::reply::with_status(warp::reply::json(&body), status))
    }
}

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Ingredients {
        pub ingredients: String,
    }

    #[derive(Serialize)]
    pub struct Recipe {
        pub title: String,
        pub ingredients: Vec<String>,
        pub instructions: Vec<String>,
    }
}

#[cfg(test)]
mod tests {
    use super::{filters, models::Ingredients, openai};
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn test_post() {
        let db = openai::Client::new("test_api_key".to_string());
        let api = filters::sandwich(db);

        let resp = request()
            .method("POST")
            .path("/generate")
            .json(&ingreds())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    fn ingreds() -> Ingredients {
        Ingredients {
            ingredients: "banana,parsley".to_owned(),
        }
    }
}
