use leptos::prelude::*;
use leptos_use::math::use_not;

use crate::{
    api::meal::delete_meal,
    components::{forms::meal_form::UpdateMealForm, modal::Modal},
    models::meal::MealWithIngredients,
};

#[component]
pub fn Meal(meal: MealWithIngredients) -> impl IntoView {
    let (deleted, set_deleted) = signal(false);
    let delete_meal_action = Action::new(|id: &i32| {
        let id = id.clone();
        async move { delete_meal(id).await }
    });
    let meal = RwSignal::new(meal);
    let update_completed = RwSignal::new(true);
    let show_update = use_not(update_completed);
    // let meal_clone = meal.clone();
    view! {
        <Show when=move || !deleted.get() fallback=|| view! {}>
            <Modal show=show_update clone:meal>
                <UpdateMealForm meal=meal completed=update_completed.write_only() />
            </Modal>
            <div class="relative w-80 max-w-sm bg-white border border-gray-200 rounded-xl shadow-sm dark:bg-gray-800 dark:border-gray-700 flex flex-col transition-all duration-300">

                // <A href=RouteUrl::EditMeal {
                // id: meal.meal.id,
                // }>
                <span
                    class="absolute top-2 left-2 z-10"
                    title="Edit meal"
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
                // </A>
                {move || {
                    let meal = meal.read();
                    let meal_id = meal.meal.id.clone();
                    view! {
                        <span
                            class="absolute top-2 right-2 z-10"
                            title="Delete day"
                            on:click= move |_| {
                                set_deleted.set(true);
                                delete_meal_action.dispatch(meal_id);
                            }
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-6 text-red-500"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
                                />
                            </svg>
                        </span>

                        // Header
                        <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                            <h5 class="text-xl font-bold text-blue-700 dark:text-blue-400 font-underline">
                                {meal.meal.name.clone()}
                            </h5>
                        </div>
                        // Image
                        <img
                            class="w-full h-48 object-cover rounded-b-none rounded-t-lg"
                            src=meal.meal.image.clone()
                            alt=meal.meal.name.clone()
                        />
                        // Footer: Ingredients
                        <div class="p-4 border-t border-gray-200 dark:border-gray-700 mt-auto">
                            <h6 class="text-md font-semibold text-gray-900 dark:text-white mb-2">
                                Ingredients
                            </h6>
                            <div class="flex flex-wrap gap-1">
                                {meal
                                    .ingredients
                                    .clone()
                                    .into_iter()
                                    .map(|ingredient| {
                                        view! {
                                            <button
                                                type="button"
                                                class="px-3 py-2 rounded-full font-semibold transition text-white bg-blue-500"
                                            >
                                                {ingredient.name.clone()}
                                                <span class="ml-2 text-xs font-normal text-gray-800 bg-gray-200 dark:bg-gray-700 dark:text-gray-100 px-2 py-1 rounded">
                                                    {ingredient.amount}
                                                </span>
                                            </button>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                }}
            </div>
        </Show>
    }
}
