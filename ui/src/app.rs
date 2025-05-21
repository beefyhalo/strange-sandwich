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
        let response = invoke("greet", args).await;
        serde_wasm_bindgen::from_value(response).unwrap()
    }));

    view! {
        main(class="@container mx-auto bg-(image:--i-like-food) bg-slate-50 min-h-screen text-gray-800 font-sans") {
            // Header + Form
            div(class="sticky top-0 z-50 backdrop-blur-lg bg-white/80 shadow-sm border-b border-slate-200 px-6 py-4") {
                h1(class="text-4xl font-extrabold text-indigo-700 tracking-tight text-center mb-4") {
                    "Strange Sandwich"
                }

                form(class="max-w-3xl mx-auto flex flex-col sm:flex-row items-center gap-4", on:submit=move |e: SubmitEvent| {
                    e.prevent_default();
                    trigger.set(());
                }) {
                    input(
                        id="search-input",
                        class="w-full sm:flex-1 px-4 py-2 rounded-xl border border-slate-300 shadow-sm focus:ring-2 focus:ring-indigo-500 focus:outline-none",
                        bind:value=name,
                        placeholder="Search for a sandwich idea..."
                    )
                    button(
                        class="px-6 py-2 bg-indigo-600 text-white rounded-xl hover:bg-indigo-700 transition font-semibold shadow-sm",
                        r#type="submit"
                    ) {
                        "Submit"
                    }
                }
            }

            // Content
            div(class="px-4 sm:px-6 py-8") {
                Suspense(fallback=|| view! {
                    div(class="text-center text-gray-500 italic py-10 animate-pulse") {
                        button(r#type="button", class="flex items-center justify-center gap-2 text-indigo-600") {
                            svg(class="size-5 animate-spin", viewBox="0 0 24 24") {
                                path(d="M12 2a10 10 0 1 0 0 20a10 10 0 0 0 0-20zm1.5 15h-3v-3h3v3zm0-4.5h-3V7h3v5.5z", fill="currentColor")
                            }
                            "Loading your delicious result..."
                        }
                    }
                }) {
                    (if let Some(recipe) = recipe.get_clone() {
                        RecipeView(recipe)
                    } else {
                        view! {}
                    })
                }
            }
        }
    }
}

#[component]
fn RecipeView(recipe: Recipe) -> View {
    view! {
        div(class="bg-white/90 backdrop-blur rounded-2xl shadow-xl p-8 max-w-3xl mx-auto space-y-6 ring-1 ring-slate-100") {
            h2(class="text-3xl font-bold text-slate-800") { (recipe.title) }
            p(class="text-gray-600 italic") { (recipe.description) }

            (if let Some(image) = recipe.image {
                view! {
                    img(src=image, alt="Recipe image", class="w-full rounded-xl shadow-md border border-slate-200")
                }
            } else {
                view! {}
            })

            // Metadata
            div(class="text-sm text-slate-500 flex flex-wrap gap-2") {
                (if let Some(prep) = recipe.prep_time { view! { span { "Prep: " (prep) } } } else { view! {} })
                (if let Some(cook) = recipe.cook_time { view! { span { "Cook: " (cook) } } } else { view! {} })
                (if let Some(total) = recipe.total_time { view! { span { "Total: " (total) } } } else { view! {} })
                (if let Some(yield_amt) = recipe.yield_amount { view! { span { "Yield: " (yield_amt) } }} else { view! {} })
            }

            // Ingredients
            div {
                h3(class="text-xl font-semibold text-slate-700 mt-6") { "Ingredients" }
                ul(class="list-disc list-inside space-y-1 text-slate-600") {
                    Indexed(list=recipe.ingredients, view=|item| {
                        view! { li { (item) } }
                    })
                }
            }

            // Article
            (if let Some(article) = recipe.article { view! {
                    p(class="text-gray-600") { (article) }
                }
            } else {
                view! {}
            })

            // Steps
            div {
                h3(class="text-xl font-semibold text-slate-700 mt-6") { "Steps" }
                ol(class="list-decimal list-inside space-y-4 text-slate-700") {
                    Indexed(list=recipe.steps, view=|step| {
                        view! {
                            li {
                                div {
                                    (step.description)
                                    (if let Some(img) = step.image {
                                        view! {
                                            img(src=img, class="mt-3 rounded-lg shadow-md")
                                        }
                                    } else { view! {} })
                                }
                            }
                        }
                    })
                }
            }

            // Nutrition
            (if let Some(nutrition) = recipe.nutrition {
                view! {
                    div(class="mt-6 text-sm text-slate-600") {
                        h4(class="text-base font-semibold text-slate-700 mb-2") { "Nutrition Facts" }
                        ul(class="grid grid-cols-2 sm:grid-cols-3 gap-1") {
                            (if let Some(cal) = nutrition.calories { view! { li { "Calories: " (cal) } } } else { view! {} })
                            (if let Some(fat) = nutrition.fat_content { view! { li { "Fat: " (fat) } } } else { view! {} })
                            (if let Some(carbs) = nutrition.carbohydrate_content { view! { li { "Carbs: " (carbs) } } } else { view! {} })
                            (if let Some(protein) = nutrition.protein_content { view! { li { "Protein: " (protein) } } } else { view! {} })
                            (if let Some(fiber) = nutrition.fiber_content { view! { li { "Fiber: " (fiber) } } } else { view! {} })
                            (if let Some(sugar) = nutrition.sugar_content { view! { li { "Sugar: " (sugar) } } } else { view! {} })
                        }
                    }
                }
            } else {
                view! {}
            })
        }
    }
}
