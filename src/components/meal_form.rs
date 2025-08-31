use crate::app::RouteUrl;
use crate::models::ingredient::IngredientForm;
use crate::models::meal::MealForm;
use leptos::prelude::*;
use leptos_router::components::A;

#[server]
pub async fn create_meal_with_ingredients(
    meal_form: MealForm,
    ingredient_forms: Vec<IngredientForm>,
) -> Result<(), ServerFnError> {
    use crate::db::*;
    use crate::models::day::*;
    use crate::models::ingredient::*;
    use crate::models::meal::*;
    use crate::schema::days;
    use crate::schema::ingredients;
    use crate::schema::meals;
    use diesel::dsl::insert_into;
    use diesel::prelude::*;

    let db = &mut use_context::<Db>()
        .ok_or(ServerFnError::new("Missing Db context"))?
        .get()
        .map_err(|_| ServerFnError::new("Failed to get Db connection"))?;
    let meal: Meal = insert_into(meals::table)
        .values(&meal_form)
        .get_result(db)
        .unwrap();
    for mut ingredient_form in ingredient_forms {
        ingredient_form.meal_id = Some(meal.id);
        insert_into(ingredients::table)
            .values(&ingredient_form)
            .execute(db)
            .unwrap();
    }
    Ok(())
}
#[component]
pub fn MealForm() -> impl IntoView {
    // Signals for meal fields
    let (name, set_name) = signal(String::new());
    let (image, set_image) = signal(String::new());
    let (recipie_url, set_recipie_url) = signal(String::new());

    // Signals for dynamic ingredient fields
    let (ingredients, set_ingredients) = signal(vec![IngredientForm {
        name: String::from(""),
        amount: 1,
        bought: false,
        meal_id: None,
    }]);

    // Add new ingredient field
    let add_ingredient = move |_| {
        set_ingredients.update(|ings| {
            ings.push(IngredientForm {
                name: String::from(""),
                amount: 1,
                bought: false,
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
    let add_meal_action = Action::new(|input: &(MealForm, Vec<IngredientForm>)| {
        let meal_form = input.0.clone();
        let ingredients = input.1.clone();
        async move { create_meal_with_ingredients(meal_form, ingredients).await }
    });

    // Handle form submission (pseudo-code, replace with your server call)
    let on_submit = move |_| {
        let meal = MealForm {
            name: name.get(),
            image: if image.get().is_empty() {
                None
            } else {
                Some(image.get())
            },
            recipie_url: if recipie_url.get().is_empty() {
                None
            } else {
                Some(recipie_url.get())
            },
        };
        let ingredients_vec = ingredients.get();
        // Call your server function to save meal and ingredients here
        add_meal_action.dispatch((meal, ingredients_vec));
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
            <form on:submit=on_submit class="space-y-6">
                <h2 class="font-bold text-2xl mb-4 text-gray-900 dark:text-white text-center">
                    Update Meal
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
                    <input
                        type="text"
                        placeholder="Image URL"
                        prop:value=image
                        on:input=move |ev| set_image(event_target_value(&ev))
                        class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
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
                                view! {
                                    <div class="flex flex-wrap gap-2 items-center bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border">
                                        <input
                                            type="text"
                                            placeholder="Ingredient name"
                                            prop:value=ing.name.clone()
                                            on:input:target=move |ev| {
                                                set_ingredients
                                                    .update(|ings| ings[idx].name = ev.target().value())
                                            }
                                            class="px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-700 dark:text-white"
                                            required
                                        />
                                        <input
                                            type="number"
                                            min="1"
                                            prop:value=ing.amount
                                            on:input=move |ev| {
                                                set_ingredients
                                                    .update(|ings| {
                                                        ings[idx].amount = event_target_value(&ev)
                                                            .parse()
                                                            .unwrap_or(1);
                                                    })
                                            }
                                            class="w-20 px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-700 dark:text-white"
                                            required
                                        />
                                        <label class="flex items-center gap-1 text-gray-700 dark:text-gray-200">
                                            <input
                                                type="checkbox"
                                                prop:checked=ing.bought
                                                on:change=move |ev| {
                                                    set_ingredients
                                                        .update(|ings| ings[idx].bought = event_target_checked(&ev))
                                                }
                                                class="accent-blue-500"
                                            />
                                            "Bought"
                                        </label>
                                        <button
                                            type="button"
                                            on:click=move |_| remove_ingredient(idx)
                                            class="ml-auto px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition"
                                        >
                                            "Remove"
                                        </button>
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
                    "Create Meal"
                </button>
            </form>
        </div>
    }
}
