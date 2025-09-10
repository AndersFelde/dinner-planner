use crate::api::week::{days_for_week, Week};
use crate::app::RouteUrl;
use crate::components::error_list;
use chrono::Datelike;
use leptos::prelude::*;
use leptos::either::Either;
use leptos_router::components::A;
use leptos_router::hooks::use_params;

use crate::components::models::ingredient::DayIngredient;
use crate::components::week::WeekQuery;

#[component]
pub fn ShoppingList() -> impl IntoView {
    let query = use_params::<WeekQuery>();
    let (week, set_week) = signal(Week::current());
    Effect::new(move || {
        if let Ok(query) = query.read().as_ref() {
            set_week(Week {
                week: query.week,
                year: query.year,
            });
        }
    });
    let days_resource = Resource::new(
        move || query.read().as_ref().ok().cloned(),
        |query| async move {
            if let Some(query) = query {
                days_for_week(Week {
                    week: query.week,
                    year: query.year,
                })
                .await
            } else {
                Err(ServerFnError::new("Could not get WeekQuery"))
            }
        },
    );

    // Local state for bought status per ingredient (keyed by day + ingredient name)

    let days_data = move || {
        days_resource.get().map(|val| {
            val.map(|days|{
                days.iter()
                .map(|day| {
                    let header = format!(
                        "{} - {:02}.{:02}",
                        day.day.date.weekday(),
                        day.day.date.day(),
                        day.day.date.month()
                    );
                    match &day.meal {
                        Some((meal, ingredients)) => Either::Left(view! {
                            <div class="mb-6 p-4 rounded-lg shadow bg-white dark:bg-gray-800">
                                <h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-1">
                                    {header}
                                </h2>
                                <h3 class="text-md font-semibold text-blue-700 dark:text-blue-300 mb-2">
                                    {meal.name.clone()}
                                </h3>
                                <ul class="flex justify-center items-center flex-wrap gap-2">
                                    {ingredients
                                        .iter()
                                        .map(|ingredient| {
                                            view! {
                                                <li>
                                                    <DayIngredient day_ingredient=ingredient.clone() />
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
                                    "No meal for this day"
                                </div>
                            </div>
                        }),
                    }
                })
                .collect::<Vec<_>>()
            })
        })
    };

    view! {
        <A href=move || {
            let week = week.get();
            format!("{}?week={}&year={}", RouteUrl::Home.to_string(), week.week, week.year)
        }>
            <button
                type="button"
                class="fixed bottom-4 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg  focus:outline-none focus:ring-2  transition flex items-center justify-center whitespace-nowrap"
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
                {move || { format!("Shopping list - Week {}", week.get().week) }}
            </span>
        </div>
        <div class="max-w-2xl mx-8 mt-8">
            <Transition fallback=move || {

                view! { <p class="text-center text-gray-500 dark:text-gray-400">"Loading..."</p> }
            }>
                <ErrorBoundary fallback=error_list>{move || days_data()}</ErrorBoundary>
            </Transition>
        </div>
    }
}
