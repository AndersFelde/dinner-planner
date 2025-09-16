use crate::api::extra_items::get_extra_items;
use crate::api::week::{days_for_week, Week};
use crate::app::RouteUrl;
use crate::components::error_list;
use chrono::Datelike;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_params, use_query_map};

use crate::components::models::extra_item::ExtraItem;
use crate::components::models::ingredient::DayIngredient;
use crate::components::week::WeekQuery;

#[component]
pub fn ShoppingList() -> impl IntoView {
    let params = use_params::<WeekQuery>();
    let query_map = use_query_map();
    let (week, set_week) = signal(Week::current());
    let show_meals = RwSignal::new(true);
    Effect::new(move || {
        if let Ok(query) = params.read().as_ref() {
            set_week(Week {
                week: query.week,
                year: query.year,
            });
        }
    });
    Effect::new(move || {
        if query_map.read().get("extra-items").is_some() {
            show_meals.set(false);
        }
    });
    let days_resource = Resource::new(
        move || params.read().as_ref().ok().cloned(),
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

    let extra_items_resource = OnceResource::new(get_extra_items());

    let extra_items_data = move || {
        extra_items_resource.get().map(|extra_items| {
            extra_items.map(|extra_items| match extra_items.len() {
                x if x > 0 => Either::Right({
                    view! {
                        <div class="mb-6 p-4 rounded-lg shadow bg-white dark:bg-gray-800">
                            <ul class="flex justify-center items-center flex-wrap gap-2">
                                {extra_items
                                    .iter()
                                    .map(|extra_item| {
                                        view! {
                                            <li>
                                                <ExtraItem extra_item=extra_item.clone() />
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }
                }),
                _ => Either::Left({
                    view! {
                        <div class="bg-green-100 border border-green-400 text-green-800 px-4 py-3 rounded relative text-center font-semibold">
                            "Noting more to buy (jippi)!"
                        </div>
                    }
                }),
            })
        })
    };

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

        <label class="inline-flex items-center cursor-pointer fixed bottom-4 left-1/2 -translate-x-1/2 z-50">
            <input
                type="checkbox"
                value=""
                class="sr-only peer"
                on:click=move |_| show_meals.update(|s| *s = !*s)
            />
            <div class=move || {
                format!(
                    "relative w-16 h-9 bg-blue-600 rounded-full transition-colors shadow-xl outline-none after:content-[''] after:absolute after:top-1 after:left-1 after:w-7 after:h-7 after:bg-white after:border after:border-gray-300 dark:after:border-gray-600 after:rounded-full after:shadow after:transition-all {}",
                    if show_meals.get() { "" } else { "after:translate-x-7" },
                )
            }></div>

        </label>

        {move || match show_meals.get() {
            true => {
                Either::Left({
                    view! {
                        <div class="flex justify-center items-center gap-4 mb-2 sticky top-0 z-10 bg-white dark:bg-gray-800 py-2 shadow">
                            <span class="font-bold text-base text-gray-900 dark:text-white">
                                {move || { format!("Shopping list - Week {}", week.get().week) }}
                            </span>
                        </div>
                        <div class="max-w-2xl mx-auto mt-8 mb-14">
                            <Transition fallback=move || {

                                view! {
                                    <p class="text-center text-gray-500 dark:text-gray-400">
                                        "Loading..."
                                    </p>
                                }
                            }>
                                <ErrorBoundary fallback=error_list>
                                    {move || days_data()}
                                </ErrorBoundary>
                            </Transition>
                        </div>
                    }
                })
            }
            false => {
                Either::Right({
                    view! {
                        <A href=RouteUrl::NewExtraItem.to_string()>
                            <button
                                type="button"
                                class="fixed bottom-19 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg  focus:outline-none focus:ring-2  transition flex items-center justify-center whitespace-nowrap"
                                title="Add meal"
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
                                        d="M12 4.5v15m7.5-7.5h-15"
                                    />
                                </svg>

                            </button>
                        </A>
                        <div class="flex justify-center items-center gap-4 mb-2 sticky top-0 z-10 bg-white dark:bg-gray-800 py-2 shadow">
                            <span class="font-bold text-base text-gray-900 dark:text-white">
                                "Shopping list - Extra items"
                            </span>
                        </div>
                        <div class="max-w-2xl mx-auto mt-8 mb-14">
                            <Transition fallback=move || {

                                view! {
                                    <p class="text-center text-gray-500 dark:text-gray-400">
                                        "Loading..."
                                    </p>
                                }
                            }>
                                <ErrorBoundary fallback=error_list>
                                    {move || extra_items_data()}
                                </ErrorBoundary>
                            </Transition>
                        </div>
                    }
                })
            }
        }}
    }
}
