use crate::api::meal::get_meals_ordered;
use crate::components::buttons::attendance::Attendance;
use crate::components::forms::day_form::DayForm;
use crate::components::forms::meal_form::CreateMealForm;
use crate::components::modal::Modal;
use crate::components::models::ingredient::DayIngredient;
use crate::components::models::receipt::Receipt;
use crate::models::days_ingredients::DayWithMealAndIngredients;
use crate::models::meal::{Meal, MealWithIngredients};
use chrono::{Datelike, Local};
use leptos::{either::Either, html::Div, prelude::*};
use leptos_use::math::use_not;
use web_sys::ScrollIntoViewOptions;

#[component]
pub fn Day(day: DayWithMealAndIngredients) -> impl IntoView {
    let today = Local::now().date_naive();
    let scroll_div_ref = NodeRef::<Div>::new();
    let is_today = today == day.day.date;
    if is_today {
        // TODO: this does not work when navigating back to a page
        Effect::new(move || {
            if let Some(node) = scroll_div_ref.get() {
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

    let s_day = RwSignal::new(day);
    let update_completed = RwSignal::new(true);
    let show_update = use_not(update_completed);
    let create_meal_completed = RwSignal::new(true);
    let show_create_meal = RwSignal::new(false);
    let new_meal: RwSignal<Option<MealWithIngredients>> = RwSignal::new(None);
    let meals_resource = OnceResource::new(get_meals_ordered());
    let meals: RwSignal<Vec<Meal>> = RwSignal::new(Vec::new());
    let view_receipts = RwSignal::new(false);

    Effect::watch(
        move || show_create_meal.get(),
        move |show_create_meal, _, _| {
            update_completed.set(*show_create_meal);
        },
        false,
    );
    Effect::watch(
        move || create_meal_completed.get(),
        move |create_meal_completed, _, _| {
            // show_update.set(*create_meal_completed);
            show_create_meal.set(!*create_meal_completed);
        },
        false,
    );
    Effect::watch(
        move || new_meal.get(),
        move |new_meal, _, _| {
            if let Some(new_meal) = new_meal {
                meals.write().push(new_meal.meal.clone())
            }
        },
        false,
    );

    Effect::watch(
        move || meals_resource.get(),
        move |r_meals, _, _| {
            if let Some(Ok(r_meals)) = r_meals {
                meals.set(r_meals.clone());
            }
        },
        true,
    );
    let receipt_data = move || {
        s_day.read().receipts.as_ref().map(|r| {
            r.iter()
                .map(|r| {
                    view! { <Receipt receipt_with_items=r.clone() /> }
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        <Modal show=Signal::derive(show_create_meal)>
            <CreateMealForm meal=new_meal completed=create_meal_completed.write_only() />
        </Modal>
        <Modal show=show_update>
            <DayForm
                day=s_day
                completed=update_completed.write_only()
                meals=meals
                create_meal=show_create_meal
            />
        </Modal>
        <Modal show=Signal::derive(view_receipts)>
            <div class="relative max-h-[80vh] overflow-y-auto">
                <button
                    class="fixed bottom-0 left-1/2 -translate-x-1/2 mb-4 px-4 py-2 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-lg shadow-md transition z-10 flex items-center gap-2"
                    on:click=move |_| view_receipts.set(false)
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="size-5"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M6 18 18 6M6 6l12 12"
                        />
                    </svg>
                    "Close"
                </button>
                {move || receipt_data}
            </div>
        </Modal>
        {move || {
            let day = s_day.get();
            let header = format!(
                "{} - {:02}.{:02}",
                day.day.date.weekday(),
                day.day.date.day(),
                day.day.date.month(),
            );
            match day.meal {
                Some((meal, ingredients)) => {
                    Either::Left(
                        view! {
                            <div
                                id=format!("day-{}", day.day.id)
                                class=card_classes
                                node_ref=scroll_div_ref
                            >

                                <span
                                    class="absolute top-2 left-2 z-10"
                                    title="Edit day"
                                    on:click=move |_| update_completed.set(false)
                                >
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

                                {if let Some(recipie_url) = meal.recipie_url {
                                    view! {
                                        <a href=recipie_url target="_blank">
                                            <span
                                                class="absolute top-2 right-11 z-10"
                                                title="View Recipe"
                                            >
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
                                                        d="M4.26 10.147a60.438 60.438 0 0 0-.491 6.347A48.62 48.62 0 0 1 12 20.904a48.62 48.62 0 0 1 8.232-4.41 60.46 60.46 0 0 0-.491-6.347m-15.482 0a50.636 50.636 0 0 0-2.658-.813A59.906 59.906 0 0 1 12 3.493a59.903 59.903 0 0 1 10.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.717 50.717 0 0 1 12 13.489a50.702 50.702 0 0 1 7.74-3.342M6.75 15a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm0 0v-3.675A55.378 55.378 0 0 1 12 8.443m-7.007 11.55A5.981 5.981 0 0 0 6.75 15.75v-1.5"
                                                    />
                                                </svg>

                                            </span>
                                        </a>
                                    }
                                        .into_any()
                                } else {
                                    view! {}.into_any()
                                }}
                                {if day.receipts.is_some() {
                                    view! {
                                        <span
                                            class="absolute top-2 right-2 z-10"
                                            title="View Recipe"
                                            on:click=move |_| view_receipts.set(true)
                                        >
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
                                    }
                                        .into_any()
                                } else {
                                    view! {}.into_any()
                                }}
                                // Header
                                <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                                        {header}
                                    </h4>
                                    <h5 class="text-xl font-bold text-blue-700 dark:text-blue-400 font-underline">
                                        {meal.name.clone()}
                                    </h5>
                                </div>
                                // Image
                                <img
                                    class="w-full h-48 object-cover rounded-b-none rounded-t-lg"
                                    src=meal.image
                                    alt=meal.name
                                />
                                // Footer: Ingredients
                                <div class="p-4 border-t border-gray-200 dark:border-gray-700 mt-auto">
                                    <h6 class="text-md font-semibold text-gray-900 dark:text-white mb-2">
                                        Ingredients
                                    </h6>
                                    <div class="flex flex-wrap gap-1">
                                        {ingredients
                                            .into_iter()
                                            .map(|ingredient| {
                                                view! { <DayIngredient day_ingredient=ingredient /> }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                </div>
                                <div class="border-t border-gray-200 dark:border-gray-700 flex justify-center items-center flex-nowrap gap-3 py-1">
                                    <Attendance day=&day.day />
                                </div>
                            </div>
                        },
                    )
                }
                None => {
                    Either::Right(
                        view! {
                            <div
                                id=format!("day-{}", day.day.id)
                                class=card_classes
                                node_ref=scroll_div_ref
                            >
                                // Header
                                <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                                        {header}
                                    </h4>
                                </div>
                                // Image area with big "+" button
                                <div
                                    class="w-full h-48 flex items-center justify-center bg-gray-100 dark:bg-gray-800 rounded-b-none rounded-t-lg"
                                    on:click=move |_| update_completed.set(false)
                                >
                                    <button
                                        class="text-6xl text-blue-400 hover:text-blue-500 dark:hover:text-blue-300 transition-colors bg-white dark:bg-gray-900 rounded-full w-20 h-20 flex items-center justify-center shadow-lg border-2 border-gray-300 dark:border-gray-700"
                                        title="Add meal"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="size-8"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M12 4.5v15m7.5-7.5h-15"
                                            />
                                        </svg>
                                    </button>
                                </div>
                                <div class="border-t border-gray-200 dark:border-gray-700 flex justify-center items-center flex-nowrap gap-3 py-1">
                                    <Attendance day=&day.day />
                                </div>
                            </div>
                        },
                    )
                }
            }
        }}
    }
}
