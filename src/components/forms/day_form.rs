use crate::api::day::upsert_day;
use crate::components::error_list;
use crate::models::day::DayForm;
use crate::models::days_ingredients::DayWithMealAndIngredients;
use crate::models::meal::Meal;
use chrono::{Datelike, NaiveDate};
use leptos::html::{Div, Input};
use leptos::prelude::*;
use leptos_use::{on_click_outside_with_options, OnClickOutsideOptions};

#[component]
pub fn DayForm(
    day: RwSignal<DayWithMealAndIngredients>,
    completed: WriteSignal<bool>,
    meals: RwSignal<Vec<Meal>>,
    create_meal: RwSignal<bool>,
) -> impl IntoView {
    let date = Signal::derive(move || day.get().day.date);
    let input_ref = NodeRef::<Input>::new();
    Effect::new(move || {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });
    let meal_search = RwSignal::new(String::new());
    let select_meal: RwSignal<Option<Meal>> = RwSignal::new(None);
    let search_active = RwSignal::new(false);
    let search_dropdown = NodeRef::<Div>::new();
    let _ = on_click_outside_with_options(
        search_dropdown,
        move |e| {
            // This is super hacky, but ignore doesnt work:/
            if !e.target().unwrap().to_string().as_string().unwrap().contains("InputElement") {
                search_active.set(false);
            }
        },
        OnClickOutsideOptions::default().ignore(["input"]),
    );
    Effect::watch(
        move || day.get().meal,
        move |meal, _, _| {
            if let Some((meal, _)) = meal {
                select_meal.set(Some(meal.clone()));
            }
        },
        true,
    );

    let meals_data = move || {
        let meals_vec = meals.get();
        let search_input = meal_search.get();
        let filtered = if !search_input.is_empty() {
            let search_input = search_input.to_lowercase();
            meals_vec
                .iter()
                .filter(|m| m.name.to_lowercase().contains(&search_input))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            meals_vec.clone()
        };
        filtered
            .iter()
            .map(|meal| {
                let meal = meal.clone();
                view! {
                    <li>
                        <button
                            type="button"
                            class=format!(
                                "inline-flex w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white {}",
                                if let Some(s_meal) = select_meal.get() {
                                    if meal.id == s_meal.id { "text-blue-500" } else { "" }
                                } else {
                                    ""
                                },
                            )
                            on:click=move |_| {
                                select_meal.set(Some(meal.clone()));
                                search_active.set(false);
                                meal_search.write().clear();
                            }
                        >
                            {meal.name.clone()}
                        </button>
                    </li>
                }
            })
            .collect::<Vec<_>>()
    };
    let add_day_action = Action::new(|day_form: &DayForm| {
        let day_form = day_form.clone();
        async move { upsert_day(day_form).await }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(meal) = select_meal.get() {
            let date = date.get();
            let day_form = DayForm {
                date,
                meal_id: Some(meal.id),
                week: date.iso_week().week() as i32,
                year: date.year(),
            };
            add_day_action.dispatch(day_form);
        }
    };

    Effect::new(move || {
        if let Some(Ok(new_day)) = add_day_action.value().get() {
            day.set(new_day);
            completed.set(true)
        }
    });

    view! {
        <div class="max-w-lg w-full mx-auto p-6 bg-white dark:bg-gray-900 rounded-xl shadow-lg">
            <form on:submit=on_submit class="space-y-6">
                <h2 class="font-bold text-2xl mb-4 text-gray-900 dark:text-white text-center">
                    Update Day
                </h2>
                <h3 class=move || {
                    format!(
                        "font-bold text-xl mb-4 text-center {}",
                        if select_meal.get().is_some() { "text-blue-500 " } else { "text-gray-500" },
                    )
                }>
                    {move || {
                        if let Some(meal) = select_meal.get() {
                            meal.name
                        } else {
                            String::from("Select a meal...")
                        }
                    }}
                </h3>
                <div class="relative space-y-3 w-80 mx-auto" id="meal-search">
                    <label
                        for="default-search"
                        class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white"
                    >
                        Search
                    </label>
                    <div class="relative">
                        <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
                            <svg
                                class="w-4 h-4 text-gray-500 dark:text-gray-400"
                                aria-hidden="true"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 20 20"
                            >
                                <path
                                    stroke="currentColor"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
                                />
                            </svg>
                        </div>
                        <input
                            type="text"
                            class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder="Search meals..."
                            prop:value=meal_search
                            bind:value=meal_search
                            on:focus=move |_| search_active.set(true)
                            node_ref=input_ref
                        />
                    </div>
                    <Show when=move || search_active.get() fallback=|| view! {}>
                        <div
                            class="absolute left-0 right-0 z-50 bg-white divide-y divide-gray-100 rounded-lg shadow-sm w-44 dark:bg-gray-700 overflow-y-auto h-50"
                            node_ref=search_dropdown
                        >
                            <ul class="py-2 text-sm text-gray-700 dark:text-gray-200">
                                <Transition fallback=move || {
                                    view! { <span>"Loading..."</span> }
                                }>
                                    <ErrorBoundary fallback=error_list>{meals_data}</ErrorBoundary>
                                </Transition>
                            </ul>
                        </div>
                    </Show>
                </div>

                <button
                    class="w-full py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition mb-2 flex items-center justify-center font-semibold"
                    on:click=move |_| { create_meal.set(true) }
                    type="button"
                >
                    "+ Add Meal"
                </button>
                <button
                    type="submit"
                    class="w-full py-2 bg-blue-500 text-white font-semibold rounded-lg hover:bg-blue-600 transition"
                >
                    "Update day"
                </button>
            </form>
        </div>
    }
}
