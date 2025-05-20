use serde::{Deserialize, Serialize};
use strange_sandwich_core::Recipe;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use sycamore::web::{Resource, Suspense, create_isomorphic_resource};
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

#[component]
pub fn App() -> View {
    let name = create_signal(String::new());
    let trigger = create_signal(());
    let recipe: Resource<Recipe> = create_isomorphic_resource(on(trigger, move || async move {
        let args = serde_wasm_bindgen::to_value(&GreetArgs {
            name: &name.get_clone(),
        })
        .unwrap();
        console_log!("Calling greet with args: {:?}", args);
        let response = invoke("greet", args).await;
        console_log!("Response: {:?}", response);
        serde_wasm_bindgen::from_value(response).unwrap()
    }));

    view! {
        main(class="@container grid mx-auto p-4") {
            h1(class="text-3xl font-bold text-center mb-4") { "Strange Sandwich" }
            form(class="row", on:submit=move |e:SubmitEvent| {e.prevent_default(); trigger.set(())}) {
                input(id="search-input", bind:value=name, placeholder="Search...")
                button(class="btn-primary", r#type="submit") { "Submit" }
            }
            Suspense(fallback=|| view! { "Loading..." }) {
                (if let Some(recipe) = recipe.get_clone() {
                    RecipeView(recipe)
                } else {
                    view! {}
                })
            }
        }
    }
}

#[component]
fn RecipeView(recipe: Recipe) -> View {
    view! {
        div(class="p-6 bg-white rounded-2xl shadow-md max-w-2xl mx-auto space-y-6") {
            h2(class="text-2xl font-bold text-gray-800") { (recipe.title) }

            (if let Some(desc) = recipe.description {
                view! {
                    p(class="text-gray-600 italic") { (desc) }
                }
            } else {
                view! {}
            })

            (if let Some(image) = recipe.image {
                view! {
                    img(src=image, alt="Recipe image", class="w-full rounded-lg shadow")
                }
            } else {
                view! {}
            })

            div(class="text-sm text-gray-500 space-x-2") {
                (if let Some(prep) = recipe.prep_time {
                    view! { span { "Prep: " (prep) } }
                } else {
                    view! {}
                })
                (if let Some(cook) = recipe.cook_time {
                    view! { span { "Cook: " (cook) } }
                } else {
                    view! {}
                })
                (if let Some(total) = recipe.total_time {
                    view! { span { "Total: " (total) } }
                } else {
                    view! {}
                })
                (if let Some(yield_amt) = recipe.yield_amount {
                    view! { span { "Yield: " (yield_amt) } }
                } else {
                    view! {}
                })
            }

            div {
                h3(class="font-semibold mt-4") { "Ingredients" }
                ul(class="list-disc list-inside text-gray-700") {
                    Indexed(list=recipe.ingredients, view=|item| {
                        view! { li { (item) } }
                    })
                }
            }

            div {
                h3(class="font-semibold mt-4") { "Steps" }
                ol(class="list-decimal list-inside text-gray-700 space-y-2") {
                    Indexed(list=recipe.steps, view=|step| {
                        view! {
                            li {
                                div {
                                    (step.description)
                                    (if let Some(img) = step.image {
                                        view! {
                                            img(src=img, class="mt-2 rounded shadow")
                                        }
                                    } else {
                                        view! {}
                                    })
                                }
                            }
                        }
                    })
                }
            }

            (if let Some(nutrition) = recipe.nutrition {
                view! {
                    div(class="mt-4 text-sm text-gray-600") {
                        h4(class="font-semibold") { "Nutrition Facts" }
                        ul {
                            (if let Some(cal) = nutrition.calories {
                                view! { li { "Calories: " (cal) } }
                            } else { view! {} })
                            (if let Some(fat) = nutrition.fat_content {
                                view! { li { "Fat: " (fat) } }
                            } else { view! {} })
                            (if let Some(carbs) = nutrition.carbohydrate_content {
                                view! { li { "Carbs: " (carbs) } }
                            } else { view! {} })
                            (if let Some(protein) = nutrition.protein_content {
                                view! { li { "Protein: " (protein) } }
                            } else { view! {} })
                            (if let Some(fiber) = nutrition.fiber_content {
                                view! { li { "Fiber: " (fiber) } }
                            } else { view! {} })
                            (if let Some(sugar) = nutrition.sugar_content {
                                view! { li { "Sugar: " (sugar) } }
                            } else { view! {} })
                        }
                    }
                }
            } else {
                view! {}
            })
        }
    }
}
