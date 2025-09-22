use crate::api::meal::get_meals_ordered;
use crate::components::forms::day_form::DayForm;
use crate::components::forms::meal_form::CreateMealForm;
use crate::components::models::ingredient::DayIngredient;
use crate::models::days_ingredients::DayWithMealAndIngredients;
use crate::models::meal::{Meal, MealWithIngredients};
use crate::components::modal::Modal;
use chrono::{Datelike, Local};
use leptos::{either::Either, html::Div, prelude::*};
use leptos_use::math::use_not;
use web_sys::ScrollIntoViewOptions;

#[component]
pub fn Day(day: DayWithMealAndIngredients) -> impl IntoView {
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

    let s_day = RwSignal::new(day);
    let update_completed = RwSignal::new(true);
    let show_update = use_not(update_completed);
    let create_meal_completed = RwSignal::new(true);
    let show_create_meal = RwSignal::new(false);
    let new_meal: RwSignal<Option<MealWithIngredients>> = RwSignal::new(None);
    let meals_resource = OnceResource::new(get_meals_ordered());
    let meals: RwSignal<Vec<Meal>> = RwSignal::new(Vec::new());

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
                                node_ref=node_ref
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
                                                class="absolute top-2 right-2 z-10"
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
                                node_ref=node_ref
                                on:click=move |_| update_completed.set(false)
                            >
                                // Header
                                <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                                    <h4 class="text-lg font-semibold text-gray-900 dark:text-white">
                                        {header}
                                    </h4>
                                </div>
                                // Image area with big "+" button
                                <div class="w-full h-48 flex items-center justify-center bg-gray-100 dark:bg-gray-800 rounded-b-none rounded-t-lg">
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
                            // Footer: Ingredients (empty)
                            </div>
                        },
                    )
                }
            }
        }}
    }
}
