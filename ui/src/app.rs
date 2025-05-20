use Default;
use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::Suspense;
use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Recipe {
    title: String,
    description: Option<String>,
}

#[component]
pub fn App() -> View {
    let name = create_signal(String::new());
    let recipe: Signal<Option<Recipe>> = create_signal(None);

    let greet = move |e: SubmitEvent| {
        e.prevent_default();
        console_log!("BLA!");

        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&GreetArgs {
                name: &name.get_clone(),
            })
            .unwrap();
            console_log!("Calling greet with args: {:?}", args);
            let response = invoke("greet", args).await;
            console_log!("Response: {:?}", response);
            recipe.set(Some(serde_wasm_bindgen::from_value(response).unwrap()))
        })
    };

    view! {
        main(class="container") {
            h1 {
                "Welcome to Tauri + Sycamore"
            }

            div(class="row") {
                a(href="https://tauri.app", target="_blank") {
                    img(src="public/tauri.svg", class="logo tauri", alt="Tauri logo")
                }
                a(href="https://sycamore.dev", target="_blank") {
                    img(src="public/sycamore.svg", class="logo sycamore", alt="Sycamore logo")
                }
            }
            p {
                "Click on the Tauri and Sycamore logos to learn more"
            }

            form(class="row", on:submit=greet) {
                input(id="greet-input", bind:value=name, placeholder="Enter a name...")
                button(r#type="submit") {
                    "Greet"
                }
            }
            Suspense(fallback=|| view! { "Loading..." }) {
                (if let Some(r) = recipe.get_clone() {
                    format!(
                        "Recipe: {:?} - {:?}",
                        r.title,
                        r.description
                    )
                } else {
                    "".to_string()
                })
            }
        }
    }
}
