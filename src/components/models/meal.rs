use leptos::prelude::*;
use leptos_router::components::A;

use crate::{api::meal::delete_meal, app::RouteUrl, models::meal::MealWithIngredients};

#[component]
pub fn Meal(meal: MealWithIngredients) -> impl IntoView {
    let (deleted, set_deleted) = signal(false);
    let delete_meal_action = Action::new(|id: &i32| {
        let id = id.clone();
        async move { delete_meal(id).await }
    });
    view! {
        <Show when=move || !deleted.get() fallback=|| view! {}>
            <div class="relative p-4 rounded-lg shadow bg-white dark:bg-gray-900">
                <A href=RouteUrl::EditMeal {
                    id: meal.meal.id,
                }
                    .redirect(RouteUrl::MealList.to_string())>
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
                <span
                    class="absolute top-2 right-2 z-10"
                    title="Delete day"
                    on:click=move |_| {
                        set_deleted.set(true);
                        delete_meal_action.dispatch(meal.meal.id.clone());
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
                <h2 class="text-lg font-bold text-gray-800 dark:text-gray-100 mb-2">
                    {meal.meal.name.clone()}
                </h2>
                <div class="mb-2 text-gray-700 dark:text-gray-200 font-semibold">Ingredients:</div>
                <ul class="flex flex-wrap items-center justify-center gap-2">
                    {meal
                        .ingredients
                        .iter()
                        .map(|ingredient| {
                            view! {
                                <li>
                                    <button
                                        type="button"
                                        class="px-3 py-2 rounded-full font-semibold transition text-white bg-blue-500"
                                    >
                                        {ingredient.name.clone()}
                                        <span class="ml-2 text-xs font-normal bg-gray-200 dark:bg-gray-700 px-2 py-1 rounded">
                                            {ingredient.amount}
                                        </span>
                                    </button>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </Show>
    }
}
