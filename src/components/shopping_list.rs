use crate::app::RouteUrl;
use crate::models::day::{Day, DayWithMeal};
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
pub async fn update_ingredient_bought(id: i32, bought: bool) -> Result<bool, ServerFnError> {
    use crate::db::*;
    use crate::models::ingredient::*;
    use crate::schema::ingredients;
    use diesel::dsl::update;
    use diesel::prelude::*;

    let db = &mut use_context::<Db>()
        .ok_or(ServerFnError::new("Missing Db context"))?
        .get()
        .map_err(|_| ServerFnError::new("Failed to get Db connection"))?;
    Ok(update(ingredients::table)
        .filter(ingredients::id.eq(id))
        .set(ingredients::bought.eq(bought))
        .get_result::<Ingredient>(db)
        .map_err(|_| ServerFnError::new("Failed to get Db connection"))?
        .bought)
}

#[component]
pub fn ShoppingList() -> impl IntoView {
    let query = use_params::<WeekQuery>();
    let days_resource = Resource::new(
        move || query.read().as_ref().ok().cloned().unwrap(),
        |query| {
            log!("Fetching days");
            days_for_week(Week {
                week: query.week,
                year: query.year,
            })
        },
    );

    // Local state for bought status per ingredient (keyed by day + ingredient name)
    let (bought_map, set_bought_map) = signal(std::collections::HashMap::<i32, bool>::new());

    let update_ingredient_action = Action::new(|(id, bought): &(i32, bool)| {
        let id = id.clone();
        let bought = bought.clone();
        async move { update_ingredient_bought(id, bought).await }
    });
    let days_data = move || {
        days_resource.get().map(|val| {
            val.unwrap()
                .iter()
                .map(|day| {
                    let header = format!(
                        "{} - {:02}.{:02}",
                        day.day.date.weekday(),
                        day.day.date.day(),
                        day.day.date.month()
                    );
                    match &day.meal {
                        Some(meal) => Either::Left(view! {
                            <div class="mb-6 p-4 rounded-lg shadow bg-white dark:bg-gray-800">
                                <h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-1">
                                    {header}
                                </h2>
                                <h3 class="text-md font-semibold text-blue-700 dark:text-blue-300 mb-2">
                                    {meal.meal.name.clone()}
                                </h3>
                                <ul class="flex justify-center items-center flex-wrap gap-2">
                                    {meal
                                        .ingredients
                                        .iter()
                                        .map(|ingredient| {
                                            let id = ingredient.id.clone();
                                            let bought = bought_map
                                                .get()
                                                .get(&id)
                                                .copied()
                                                .unwrap_or(ingredient.bought);
                                            view! {
                                                <li>
                                                    <button
                                                        type="button"
                                                        class=format!(
                                                            "px-3 py-2 rounded-full font-semibold transition {}",
                                                            if bought {
                                                                "bg-green-500 text-white hover:bg-green-600"
                                                            } else {
                                                                "bg-red-500 text-white hover:bg-red-600"
                                                            },
                                                        )
                                                        on:click=move |_| {
                                                            set_bought_map
                                                                .update(|map| {
                                                                    map.insert(id.clone(), !bought);
                                                                    update_ingredient_action.dispatch((id, !bought));
                                                                });
                                                        }
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
                        }),
                        None => Either::Right(view! {
                            <div class="mb-6 p-4 rounded-lg shadow bg-white dark:bg-gray-800">
                                <h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-1">
                                    {header}
                                </h2>
                                <div class="text-gray-500 dark:text-gray-400">
                                    No meal for this day
                                </div>
                            </div>
                        }),
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        <A href=move || {
            let query = query.read().as_ref().ok().cloned().unwrap();
            format!("{}?week={}&year={}", RouteUrl::Home.to_string(), query.week, query.year)
        }>
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
        <div class="flex justify-center items-center gap-4 mb-2 sticky top-0 z-10 bg-white dark:bg-gray-800 py-2 shadow">
            <span class="font-bold text-base text-gray-900 dark:text-white">
                {move || {
                    format!(
                        "Shopping list - Week {}",
                        query.read().as_ref().ok().cloned().unwrap().week,
                    )
                }}
            </span>
        </div>
        <div class="max-w-2xl mx-8 mt-8">
            <Transition fallback=move || {
                view! { <p class="text-center text-gray-500 dark:text-gray-400">"Loading..."</p> }
            }>
                {move || days_data()}
            </Transition>
        </div>
    }
}
