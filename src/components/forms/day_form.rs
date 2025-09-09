use crate::api::day::{get_day, upsert_day};
use crate::api::meal::get_meals;
use crate::app::RouteUrl;
use crate::components::error_list;
use crate::models::day::DayForm;
use chrono::{Datelike, NaiveDate};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate, use_params_map};

// TODO: if date allready exists, edit
#[component]
pub fn DayForm() -> impl IntoView {
    let params = use_params_map();
    let (date, set_date) = signal(String::new());
    let (meal_id, set_meal_id) = signal(-1 as i32);
    let navigate = use_navigate();

    let day_resource = Resource::new(
        move || {
            params
                .read()
                .get("id")
                .and_then(|id| id.parse::<i32>().ok())
        },
        move |id| async move {
            match id {
                Some(id) => get_day(id).await.map(Some),
                None => Ok(None),
            }
        },
    );

    let meals_resource = OnceResource::new(get_meals());

    // Effect to redirect after success

    // let day = Resource::new(move || day.get(), move |day| async move { get_day(id).await });
    Effect::new({
        let set_meal_id = set_meal_id.clone();
        let set_date = set_date.clone();
        move || {
            let day_resource = day_resource.get();
            if let Some(Ok(Some(day))) = day_resource.clone() {
                if let Some(id) = day.meal_id {
                    set_meal_id(id);
                } else if let Some(Ok(meals)) = meals_resource.get() {
                    if let Some(first_meal) = meals.first() {
                        set_meal_id(first_meal.id);
                    }
                }
            }
            if let Some(Ok(Some(day))) = day_resource {
                set_date(day.date.format("%d-%m-%Y").to_string());
            }
        }
    });
    let meals_data = move || {
        if let Some(Ok(Some(day))) = day_resource.get() {
            meals_resource.get().map(|meals| {
                meals.map(|meals| {
                    meals
                .iter()
                .map(|meal| {
                    view! {
                        <option selected=day.meal_id.unwrap_or_else(|| -1) == meal.id value=meal.id>
                            {meal.name.clone()}
                        </option>
                    }
                })
                .collect::<Vec<_>>()
                })
            })
        } else {
            None
        }
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
        if let Some(Ok(_)) = add_day_action.value().get() {
            navigate(&RouteUrl::Home.to_string(), Default::default());
        }
    });

    let location = use_location();

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
                            let _ = event_target_value(&ev).parse().map(|id| set_meal_id(id));
                        }
                        class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-800 dark:text-white"
                        required
                    >
                        <Transition fallback=move || {
                            view! { <span>"Loading..."</span> }
                        }>
                            <ErrorBoundary fallback=error_list>{move || meals_data}</ErrorBoundary>
                        </Transition>
                    </select>
                </div>

                <A
                    href=move || RouteUrl::NewMeal.redirect(location.pathname.get())
                    attr:class="w-full py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition mb-2 flex items-center justify-center font-semibold"
                >
                    "+ Add Meal"
                </A>
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
