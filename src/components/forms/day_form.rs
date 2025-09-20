use crate::api::day::{get_day, upsert_day};
use crate::api::meal::get_meals;
use crate::app::RouteUrl;
use crate::components::error_list;
use crate::components::forms::meal_form::CreateMealForm;
use crate::components::modal::Modal;
use crate::models::day::DayForm;
use crate::models::days_ingredients::DayWithMealAndIngredients;
use crate::models::meal::Meal;
use crate::models::meal::MealWithIngredients;
use chrono::{Datelike, Local, NaiveDate};
use leptos::html::Select;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate, use_params_map};
use leptos_use::math::use_not;

#[component]
pub fn DayForm(
    day: RwSignal<DayWithMealAndIngredients>,
    completed: WriteSignal<bool>,
    meals: RwSignal<Vec<Meal>>,
    create_meal: RwSignal<bool>,
) -> impl IntoView {
    let (date, set_date) = signal(String::new());
    let (meal_id, set_meal_id) = signal(-1 as i32);
    let input_ref = NodeRef::<Select>::new();
    Effect::new(move || {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    // let meals_resource = OnceResource::new(get_meals());

    // Effect::watch(
    //     move || meals_resource.get(),
    //     move |r_meals, _, _| {
    //         if let Some(Ok(r_meals)) = r_meals {
    //             meals.set(r_meals.clone());
    //         }
    //     },
    //     true,
    // );
    let meals_data = move || {
        meals
            .get()
            .iter()
            .map(|meal| {
                view! {
                    <option
                        selected=day.get().day.meal_id.unwrap_or_else(|| -1) == meal.id
                        value=meal.id
                    >
                        {meal.name.clone()}
                    </option>
                }
            })
            .collect::<Vec<_>>()
        // meals_resource.get().map(|meals| {
        //     meals.map(|meals| {
        //         meals
        //             .iter()
        //             .map(|meal| {
        //                 view! {
        //                     <option
        //                         selected=day.get().day.meal_id.unwrap_or_else(|| -1) == meal.id
        //                         value=meal.id
        //                     >
        //                         {meal.name.clone()}
        //                     </option>
        //                 }
        //             })
        //             .collect::<Vec<_>>()
        //     })
        // })
    };
    let add_day_action = Action::new(|day_form: &DayForm| {
        let day_form = day_form.clone();
        async move { upsert_day(day_form).await }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Ok(date) = NaiveDate::parse_from_str(&date.get(), "%d-%m-%Y") {
            let meal_id = meal_id.get();
            let day_form = DayForm {
                date,
                meal_id: Some(meal_id),
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
        {move || {
            let day = day.get();
            set_date.set(day.day.date.format("%d-%m-%Y").to_string());
            set_meal_id.set(day.day.meal_id.unwrap_or(-1));
            view! {
                <div class="max-w-lg w-full mx-auto p-6 bg-white dark:bg-gray-900 rounded-xl shadow-lg">
                    <form on:submit=on_submit class="space-y-6">
                        <h2 class="font-bold text-2xl mb-4 text-gray-900 dark:text-white text-center">
                            Update Day
                        </h2>
                        <div class="space-y-3">
                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1 text-left">
                                Date
                            </label>
                            <input
                                type="text"
                                placeholder="Date"
                                prop:value=date
                                on:input=move |ev| set_date(event_target_value(&ev))
                                class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                                required
                            />
                            <label class="block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1 text-left">
                                Meal
                            </label>
                            <select
                                prop:value=meal_id
                                on:change=move |ev| {
                                    let _ = event_target_value(&ev)
                                        .parse()
                                        .map(|id| set_meal_id(id));
                                }
                                class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                                node_ref=input_ref
                                required
                            >
                                <Transition fallback=move || {
                                    view! { <span>"Loading..."</span> }
                                }>
                                    <ErrorBoundary fallback=error_list>
                                        {move || meals_data}
                                    </ErrorBoundary>
                                </Transition>
                            </select>
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
        }}
    }
}
