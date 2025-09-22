use crate::api::extra_items::get_extra_items_not_bought;
use crate::api::week::{days_for_week, Week};
use crate::app::{GlobalStateStoreFields, RouteUrl};
use crate::components::error_list;
use crate::components::forms::extra_item_form::CreateExtraItemForm;
use crate::models::days_ingredients::DayWithMealAndIngredients;
use chrono::{Datelike, Local};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_use::math::use_not;
use reactive_stores::Store;

use crate::app::GlobalState;
use crate::components::modal::Modal;
use crate::components::models::extra_item::ExtraItem;
use crate::components::models::ingredient::DayIngredient;
use crate::models::extra_item::ExtraItem;

#[component]
pub fn ShoppingList() -> impl IntoView {
    // let params = use_params::<WeekQuery>();
    // let query_map = use_query_map();
    // let (week, set_week) = signal(Week::current());
    // let show_meals = RwSignal::new(true);
    let extra_items: RwSignal<Vec<ExtraItem>> = RwSignal::new(Vec::new());
    let days: RwSignal<Vec<DayWithMealAndIngredients>> = RwSignal::new(Vec::new());
    let state = expect_context::<Store<GlobalState>>();
    let extra_items_count = state.extra_items_count();
    let week_ingredients_count = state.week_ingredients_count();
    let now = Local::now().date_naive().iso_week();

    // Effect::new(move || {
    //     if query_map.read().get("extra-items").is_some() {
    //         show_meals.set(false);
    //     }
    // });
    let days_resource = OnceResource::new(days_for_week(Week {
        week: now.week(),
        year: now.year(),
    }));

    let extra_items_resource = OnceResource::new(get_extra_items_not_bought());
    let create_extra_item_completed = RwSignal::new(true);
    let show_create_extra_item = use_not(create_extra_item_completed);
    let new_extra_item: RwSignal<Option<ExtraItem>> = RwSignal::new(None);
    Effect::watch(
        move || extra_items_resource.get(),
        move |r_extra_items, _, _| {
            if let Some(Ok(r_extra_items)) = r_extra_items {
                extra_items.set(r_extra_items.clone());
            }
        },
        true,
    );
    Effect::watch(
        move || days_resource.get(),
        move |r_days, _, _| {
            if let Some(Ok(r_days)) = r_days {
                days.set(r_days.into());
            }
        },
        true,
    );
    Effect::watch(
        move || new_extra_item.get(),
        move |new_extra_item, _, _| {
            if let Some(new_extra_item) = new_extra_item {
                extra_items.write().push(new_extra_item.clone())
            }
        },
        false,
    );
    Effect::watch(
        move || extra_items.get(),
        move |extra_items, _, _| extra_items_count.set(extra_items.len()),
        false,
    );

    Effect::watch(
        move || days.get(),
        move |days, _, _| {

            let num_days = days.iter().filter(|day| {
                if let Some((_, ingredients)) = day.meal.as_ref() {
                    if ingredients.iter().any(|i| i.bought == false) {
                        return true;
                    }
                }
                false
            })
            .count();
            week_ingredients_count.set(num_days)
        },
        false,
    );

    let extra_items_data = move || {
        let extra_items = extra_items.get();
        let l = extra_items.len();
        view! {
            // <div class="mb-6 p-4 rounded-lg shadow bg-white dark:bg-gray-800">
            <Show when=move || { l > 0 } fallback=|| view! {}>
                <div class="flex items-center justify-center mx-5 ">
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4 w-full max-w-3xl">
                        {extra_items
                            .iter()
                            .map(|extra_item| {
                                view! { <ExtraItem extra_item=extra_item.clone() /> }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </Show>
        }
    };

    // Local state for bought status per ingredient (keyed by day + ingredient name)

    let days_data = move || {
        // days_resource.get().map(|val| {
        //     val.map(|days|{
        days.get()
            .iter()
            .filter(|day| {
                if let Some((_, ingredients)) = day.meal.as_ref() {
                    if ingredients.iter().any(|i| i.bought == false) {
                        return true;
                    }
                }
                false
            })
            .map(|day| {
                let (meal, ingredients) = day.meal.as_ref().unwrap();
                // if let Some((meal, ingredients)) = day.meal.as_ref() {
                //     if ingredients.iter().any(|i|i.bought==false){
                let header = format!("{} - {}", day.day.date.weekday(), meal.name.clone());
                // let (meal, ingredients) = day.meal.as_ref().unwrap().clone();
                view! {
                    <div class="mb-6 p-4 rounded-lg shadow bg-white dark:bg-gray-800">
                        <h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-1">
                            {header}
                        </h2>
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
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <A href=RouteUrl::Home.to_string()>
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
        <button
            type="button"
            class="fixed bottom-19 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg  focus:outline-none focus:ring-2  transition flex items-center justify-center whitespace-nowrap"
            title="Add meal"
            on:click=move |_| {
                create_extra_item_completed.set(false);
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
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>

        </button>
        <Modal show=show_create_extra_item>
            <CreateExtraItemForm
                extra_item=new_extra_item
                completed=create_extra_item_completed.write_only()
            />
        </Modal>

        <div class="flex justify-center items-center gap-4 mb-2 sticky top-0 z-10 bg-white dark:bg-gray-800 py-2 shadow">
            <span class="font-bold text-base text-gray-900 dark:text-white">
                {move || { format!("Shopping list - Week {}", now.week()) }}
            </span>
        </div>
        <div class="flex flex-col gap-4 items-center justify-center max-w-2xl mx-auto mt-8 mb-14">
            <Transition fallback=move || {

                view! { <p class="text-center text-gray-500 dark:text-gray-400">"Loading..."</p> }
            }>
                <ErrorBoundary fallback=error_list>{move || extra_items_data()}</ErrorBoundary>
            </Transition>
            <Transition fallback=move || {

                view! { <p class="text-center text-gray-500 dark:text-gray-400">"Loading..."</p> }
            }>
                <div class="flex items-center justify-center mx-5 ">
                    <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 gap-4 w-full max-w-3xl">
                        <ErrorBoundary fallback=error_list>{move || days_data()}</ErrorBoundary>
                    </div>
                </div>
            </Transition>
        </div>
    }
}
