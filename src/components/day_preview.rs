use crate::app::RouteUrl;
use crate::models::day::*;
use crate::models::ingredient::*;
use crate::models::meal::*;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use leptos::{
    either::Either,
    html::{Div, HtmlElement, Input},
    logging::log,
    prelude::*,
};
use leptos_router::components::A;
use web_sys::ScrollIntoViewOptions;

#[component]
pub fn IngredientBox(ingredient: Ingredient) -> impl IntoView {
    let box_classes = if ingredient.bought {
        "inline-block px-3 py-1 m-1 rounded-full bg-green-200 text-green-900 border border-green-400 shadow-sm text-sm font-medium"
    } else {
        "inline-block px-3 py-1 m-1 rounded-full bg-red-100 text-red-900 border border-red-300 shadow-sm text-sm font-medium"
    };

    let label = if ingredient.amount > 1 {
        format!("{} x{}", ingredient.name, ingredient.amount)
    } else {
        ingredient.name.clone()
    };

    view! { <span class=box_classes>{label}</span> }
}

#[component]
pub fn DayPreview(day: DayWithMeal) -> impl IntoView {
    let today = Local::now().date_naive();
    let node_ref = NodeRef::<Div>::new();
    let is_today = today == day.day.date;
    if is_today {
        // TODO: this does not work when navigating back to a page
        Effect::new(move || {
            if let Some(node) = node_ref.get() {
                let options = ScrollIntoViewOptions::new();
                options.set_behavior(web_sys::ScrollBehavior::Auto);
                options.set_block(web_sys::ScrollLogicalPosition::Center);
                options.set_inline(web_sys::ScrollLogicalPosition::Center);
                node.scroll_into_view_with_scroll_into_view_options(&options);
            }
        });
    }
    let card_classes = if is_today {
        "relative w-80 max-w-sm bg-white border-4 rounded-xl shadow-lg border-gray-200 rounded-xl shadow-sm dark:bg-gray-800 dark:border-blue-500 flex flex-col transition-all duration-300"
    } else {
        "relative w-80 max-w-sm bg-white border border-gray-200 rounded-xl shadow-sm dark:bg-gray-800 dark:border-gray-700 flex flex-col transition-all duration-300"
    };

    let header = format!(
        "{} - {:02}.{:02}",
        day.day.date.weekday(),
        day.day.date.day(),
        day.day.date.month()
    );
    match day.meal {
        Some(meal) => Either::Left(view! {
            <div class=card_classes node_ref=node_ref>

                <A href=RouteUrl::EditDay {
                    id: day.day.id,
                }>
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

                {if let Some(recipie_url) = meal.meal.recipie_url {
                    view! {
                        <a href=recipie_url target="_blank">
                            <span class="absolute top-2 right-2 z-10" title="View Recipe">
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
                                        d="M12 7.5h1.5m-1.5 3h1.5m-7.5 3h7.5m-7.5 3h7.5m3-9h3.375c.621 0 1.125.504 1.125 1.125V18a2.25 2.25 0 0 1-2.25 2.25M16.5 7.5V18a2.25 2.25 0 0 0 2.25 2.25M16.5 7.5V4.875c0-.621-.504-1.125-1.125-1.125H4.125C3.504 3.75 3 4.254 3 4.875V18a2.25 2.25 0 0 0 2.25 2.25h13.5M6 7.5h3v3H6v-3Z"
                                    />
                                </svg>

                            </span>
                        </a>
                    }
                        .into_any()
                } else {
                    view! {}.into_any()
                }}
                // Header
                <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">{header}</h4>
                    <h5 class="text-xl font-bold text-blue-700 dark:text-blue-400 font-underline">
                        {meal.meal.name.clone()}
                    </h5>
                </div>
                // Image
                <img
                    class="w-full h-48 object-cover rounded-b-none rounded-t-lg"
                    src=meal.meal.image
                    alt=meal.meal.name
                />
                // Footer: Ingredients
                <div class="p-4 border-t border-gray-200 dark:border-gray-700 mt-auto">
                    <h6 class="text-md font-semibold text-gray-900 dark:text-white mb-2">
                        Ingredients
                    </h6>
                    <div class="flex flex-wrap gap-1">
                        {meal
                            .ingredients
                            .into_iter()
                            .map(|ingredient| view! { <IngredientBox ingredient=ingredient /> })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        }),
        None => Either::Right(view! {
            <A href=RouteUrl::EditDay {
                id: day.day.id,
            }>
                <div class=card_classes node_ref=node_ref>
                    // Header
                    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                        <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                            {header}
                        </h4>
                    </div>
                    // Image area with big "+" button
                    <div class="w-full h-48 flex items-center justify-center bg-gray-100 dark:bg-gray-800 rounded-b-none rounded-t-lg">
                        <button
                            class="text-6xl text-gray-400 dark:text-blue-400 hover:text-blue-500 dark:hover:text-blue-300 transition-colors bg-white dark:bg-gray-900 rounded-full w-20 h-20 flex items-center justify-center shadow-lg border-2 border-gray-300 dark:border-gray-700"
                            title="Add meal"
                        >
                            "+"
                        </button>
                    </div>
                // Footer: Ingredients (empty)
                </div>
            </A>
        }),
    }
}
