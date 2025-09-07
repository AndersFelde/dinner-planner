use crate::app::RouteUrl;
use crate::models::day::{Day, DayWithMeal};
use crate::models::meal::MealWithIngredients;
use leptos::prelude::*;
use leptos::{either::Either, logging::log};
use leptos_router::components::A;
use leptos_router::hooks::{use_params, use_query};

use crate::components::day_preview::*;

use chrono::{Datelike, Local, NaiveDate, Weekday};

use crate::components::week::{days_for_week, Week, WeekQuery};
use leptos::Params;
use leptos_router::params::Params;

#[server]
pub async fn get_all_meals_with_ingredients() -> Result<Vec<MealWithIngredients>, ServerFnError> {
    use crate::db::*;
    use crate::models::ingredient::*;
    use crate::models::meal::*;
    use crate::schema::ingredients;
    use crate::schema::meals;
    use diesel::prelude::*;

    let db = &mut use_context::<Db>()
        .ok_or(ServerFnError::new("Missing Db context"))?
        .get()
        .map_err(|_| ServerFnError::new("Failed to get Db connection"))?;
    let mut meals_with_ingredients = vec![];
    for meal in meals::table
        .select(Meal::as_select())
        .load::<Meal>(db)
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?
    {
        meals_with_ingredients.push(MealWithIngredients {
            meal: meal.clone(),
            ingredients: ingredients::table
                .filter(ingredients::meal_id.eq(meal.id))
                .load::<Ingredient>(db)?,
        });
    }
    Ok(meals_with_ingredients)
}

#[component]
pub fn MealList() -> impl IntoView {
    let meals_resource = OnceResource::new(get_all_meals_with_ingredients());
    let meals_data = move || {
        meals_resource.get().map(|val| {
            val.unwrap()
                .iter()
                .map(|meal| {
                    view! {
                        <li class="mb-6">
                            <div class="relative p-4 rounded-lg shadow bg-white dark:bg-gray-900">
                                <A href=RouteUrl::EditMeal {
                                    id: meal.meal.id,
                                }.redirect(RouteUrl::MealList.to_string())>
                                    <span class="absolute top-2 left-2 z-10" title="Edit day">
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="size-6 text-blue-500"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10"
                                            />
                                        </svg>
                                    </span>
                                </A>
                                <h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-2">
                                    {meal.meal.name.clone()}
                                </h2>
                                <div class="mb-2 text-gray-700 dark:text-gray-200 font-semibold">
                                    Ingredients:
                                </div>
                                <ul class="flex flex-wrap gap-2">
                                    {meal
                                        .ingredients
                                        .iter()
                                        .map(|ingredient| {
                                            view! {
                                                <li>
                                                    <button
                                                        type="button"
                                                        class=format!(
                                                            "px-3 py-2 rounded-full font-semibold transition {}",
                                                            if ingredient.bought {
                                                                "bg-green-500 text-white hover:bg-green-600"
                                                            } else {
                                                                "bg-red-500 text-white hover:bg-red-600"
                                                            },
                                                        )
                                                    >
                                                        {ingredient.name.clone()}
                                                        <span class="ml-2 text-xs font-normal bg-gray-200 dark:bg-gray-700 px-2 py-1 rounded">
                                                            {ingredient.amount}
                                                        </span>
                                                    </button>
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </ul>
                            </div>
                        </li>
                    }
                })
                .collect::<Vec<_>>()
        })
    };
    view! {
        <A href=RouteUrl::Home.to_string()>
            <button
                type="button"
                class="fixed bottom-4 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-400 transition flex items-center justify-center whitespace-nowrap"
                title="View shopping list"
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
                        d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18"
                    />
                </svg>

            </button>
        </A>
        <Transition fallback=move || {
            view! { <p class="text-center text-gray-500 dark:text-gray-400">"Loading..."</p> }
        }>
            <ul class="max-w-2xl mx-auto mt-8">{move || meals_data()}</ul>
        </Transition>
    }
}
