use crate::api::meal::{create_meal_with_ingredients, get_meal, update_meal_with_ingredients};
use crate::app::RouteUrl;
use crate::components::error_list;
use crate::models::ingredient::IngredientForm;
use crate::models::meal::{Meal, MealForm, MealWithIngredients};
use leptos::either::Either;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map, use_query_map};

#[component]
pub fn UpdateMealForm() -> impl IntoView {
    let params = use_params_map();
    let meal_resource = Resource::new(
        move || {
            params
                .read()
                .get("id")
                .and_then(|id| id.parse::<i32>().ok())
        },
        move |id| async move {
            match id {
                Some(id) => get_meal(id).await.map(Some),
                None => Ok(None),
            }
        },
    );
    let add_meal_action = Action::new(|input: &(Meal, Vec<IngredientForm>)| {
        let meal = input.0.clone();
        let ingredients = input.1.clone();
        log!("Ingredients: {ingredients:?}");
        async move { update_meal_with_ingredients(meal, ingredients).await }
    });
    let query = use_query_map();
    let navigate = use_navigate();
    let redirect = move || {
        if let Some(url) = query.read().get("redirect") {
            navigate(&url, Default::default());
        } else {
            navigate(&RouteUrl::Home.to_string(), Default::default());
        }
    };
    Effect::new(move || {
        if let Some(Ok(_)) = add_meal_action.value().get() {
            redirect();
        }
    });
    // match meal_resource.get() {
    //     Some(Ok(meal)) => Either::Left({
    let meal_form = move || {
        meal_resource.get().map(|meal| {
            meal.map(|meal| {
                meal.map(|meal| {
                    let id = meal.meal.id.clone();
                    let on_submit =
                        move |meal_form: MealForm, ingredient_forms: Vec<IngredientForm>| {
                            add_meal_action.dispatch((
                                Meal {
                                    id,
                                    name: meal_form.name,
                                    image: meal_form.image,
                                    recipie_url: meal_form.recipie_url,
                                },
                                ingredient_forms,
                            ));
                        };
                    view! { <MealForm meal=Some(meal) on_submit=on_submit /> }
                })
            })
        });
    };
    view! {
        <Suspense fallback=move || {
            view! { <span>"Loading..."</span> }
        }>
            <ErrorBoundary fallback=error_list>{meal_form}</ErrorBoundary>
        </Suspense>
    }
    // if let Some(Ok(meal)) = meal_resource.get() {
}
#[component]
pub fn CreateMealForm() -> impl IntoView {
    let add_meal_action = Action::new(|input: &(MealForm, Vec<IngredientForm>)| {
        let meal_form = input.0.clone();
        let ingredients = input.1.clone();
        async move { create_meal_with_ingredients(meal_form, ingredients).await }
    });
    let query = use_query_map();
    let navigate = use_navigate();
    // TODO: move into meal form
    let redirect = move || {
        if let Some(url) = query.read().get("redirect") {
            navigate(&url, Default::default());
        } else {
            navigate(&RouteUrl::Home.to_string(), Default::default());
        }
    };
    Effect::new(move || {
        if let Some(Ok(_)) = add_meal_action.value().get() {
            redirect();
        }
    });

    let on_submit = move |meal_form: MealForm, ingredient_forms: Vec<IngredientForm>| {
        add_meal_action.dispatch((meal_form, ingredient_forms));
    };
    view! { <MealForm meal=None on_submit=on_submit /> }
}

#[component]
pub fn MealForm<A>(meal: Option<MealWithIngredients>, on_submit: A) -> impl IntoView
where
    A: Fn(MealForm, Vec<IngredientForm>) + 'static,
{
    // Signals for meal fields
    let (name, image, recipie_url, ingredients) = if let Some(meal) = meal.clone() {
        (
            meal.meal.name,
            meal.meal.image,
            meal.meal.recipie_url.unwrap_or_default(),
            meal.ingredients
                .iter()
                .map(|ingredient| IngredientForm {
                    name: ingredient.name.clone(),
                    amount: ingredient.amount,
                    meal_id: Some(ingredient.meal_id),
                })
                .collect(),
        )
    } else {
        (
            String::new(),
            String::new(),
            String::new(),
            vec![IngredientForm {
                name: String::from(""),
                amount: 1,
                meal_id: None,
            }],
        )
    };
    let (name, set_name) = signal(name);
    let (image, set_image) = signal(image);
    let (recipie_url, set_recipie_url) = signal(recipie_url);

    // Signals for dynamic ingredient fields
    let (ingredients, set_ingredients) = signal(ingredients);

    // Add new ingredient field
    let add_ingredient = move |_| {
        set_ingredients.update(|ings| {
            ings.push(IngredientForm {
                name: String::from(""),
                amount: 1,
                meal_id: None,
            });
        });
    };

    // Remove ingredient field
    let remove_ingredient = move |idx: usize| {
        set_ingredients.update(|ings| {
            if ings.len() > 1 {
                ings.remove(idx);
            }
        });
    };

    // Handle form submission (pseudo-code, replace with your server call)
    let form_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let meal = MealForm {
            name: name.get(),
            image: image.get(),
            recipie_url: if recipie_url.get().is_empty() {
                None
            } else {
                Some(recipie_url.get())
            },
        };
        let ingredients_vec = ingredients.get();
        // Call your server function to save meal and ingredients here
        on_submit(meal, ingredients_vec);
    };

    let action_name = {
        if meal.is_some() {
            "Update Meal"
        } else {
            "Create Meal"
        }
    };

    view! {
        <div class="max-w-lg mx-auto mt-8 p-6 bg-white dark:bg-gray-900 rounded-xl shadow-lg">
            <A href=RouteUrl::Home attr:class="text-blue-500 hover:underline mb-4 inline-block">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="size-6"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18"
                    />
                </svg>

            </A>
            <form on:submit=form_submit class="space-y-6">
                <h2 class="font-bold text-2xl mb-4 text-gray-900 dark:text-white text-center">
                    {action_name}
                </h2>
                <div class="space-y-3">
                    <input
                        type="text"
                        placeholder="Meal name"
                        prop:value=name
                        on:input=move |ev| set_name(event_target_value(&ev))
                        class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                        required
                    />
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1 text-left">
                        *will be autogenerated
                    </label>
                    <input
                        type="url"
                        placeholder="Image URL"
                        prop:value=image
                        on:input=move |ev| set_image(event_target_value(&ev))
                        class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                        prop:required=false
                    />
                    <input
                        type="text"
                        placeholder="Recipe URL"
                        prop:value=recipie_url
                        on:input=move |ev| set_recipie_url(event_target_value(&ev))
                        class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                    />
                </div>

                <h3 class="font-semibold text-lg mb-2 text-gray-900 dark:text-white">
                    Ingredients
                </h3>
                <div class="space-y-3">
                    {move || {
                        ingredients
                            .get()
                            .iter()
                            .enumerate()
                            .map(|(idx, ing)| {
                                // ...existing code...
                                view! {
                                    <div class="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border mb-2 flex flex-nowrap gap-2 items-center">
                                        // Ingredient name input
                                        // <Show
                                        // when=move || { ingredients.get().len() > 1 }
                                        // fallback=|| ()
                                        // >
                                        <button
                                            type="button"
                                            on:click=move |_| remove_ingredient(idx)
                                            class=move || {
                                                format!(
                                                    "px-3 py-1 text-white rounded  transition {}",
                                                    if ingredients.get().len() > 1 {
                                                        "bg-red-500 hover:bg-red-600"
                                                    } else {
                                                        "bg-gray-500"
                                                    },
                                                )
                                            }
                                        >
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                stroke-width="1.5"
                                                stroke="currentColor"
                                                class="size-6"
                                            >
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
                                                />
                                            </svg>

                                        </button>
                                        // </Show>
                                        <input
                                            type="text"
                                            placeholder="Ingredient name"
                                            prop:value=ing.name.clone()
                                            on:input:target=move |ev| {
                                                set_ingredients
                                                    .update(|ings| ings[idx].name = ev.target().value())
                                            }
                                            class="px-3 py-2 w-40 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-700 dark:text-white"
                                            required
                                        />
                                        // Amount display and buttons
                                        <span class="flex items-center gap-1">
                                            <button
                                                type="button"
                                                class="px-2 py-1 bg-gray-200 rounded hover:bg-gray-300"
                                                on:click=move |_| {
                                                    set_ingredients
                                                        .update(|ings| {
                                                            if ings[idx].amount > 1 {
                                                                ings[idx].amount -= 1;
                                                            }
                                                        })
                                                }
                                            >
                                                "-"
                                            </button>
                                            <span class="px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 dark:text-white min-w-[2.5rem] text-center">
                                                {ing.amount}
                                            </span>
                                            <button
                                                type="button"
                                                class="px-2 py-1 bg-gray-200 rounded hover:bg-gray-300"
                                                on:click=move |_| {
                                                    set_ingredients
                                                        .update(|ings| {
                                                            ings[idx].amount += 1;
                                                        })
                                                }
                                            >
                                                "+"
                                            </button>
                                        </span>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
                <button
                    type="button"
                    on:click=add_ingredient
                    class="w-full py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition mb-2"
                >
                    "+ Add Ingredient"
                </button>
                <button
                    type="submit"
                    class="w-full py-2 bg-blue-500 text-white font-semibold rounded-lg hover:bg-blue-600 transition"
                >
                    {action_name}
                </button>
            </form>
        </div>
    }
}
