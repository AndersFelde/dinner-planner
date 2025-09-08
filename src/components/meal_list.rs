use crate::api::meal::get_all_meals_with_ingredients;
use crate::app::RouteUrl;
use crate::components::models::meal::Meal;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn MealList() -> impl IntoView {
    let meals_resource = OnceResource::new(get_all_meals_with_ingredients());
    let meals_data = move || {
        meals_resource.get().map(|val| {
            val.unwrap()
                .iter()
                .map(|meal| {
                    view! {
                        <li class="mb-6">
                            <Meal meal=meal.clone() />

                        </li>
                    }
                })
                .collect::<Vec<_>>()
        })
    };
    view! {
        <A href=RouteUrl::Home.to_string()>
            <button
                type="button"
                class="fixed bottom-4 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-400 transition flex items-center justify-center whitespace-nowrap"
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
        <A href=RouteUrl::NewMeal.to_string()>
            <div class="flex justify-center mt-8">
                <button
                    type="button"
                    class="w-16 h-16 rounded-full bg-green-500 text-white text-4xl flex items-center justify-center shadow-lg hover:bg-green-600 transition"
                    title="Add meal"
                >
                    "+"
                </button>
            </div>
        </A>
        <Transition fallback=move || {
            view! { <p class="text-center text-gray-500 dark:text-gray-400">"Loading..."</p> }
        }>
            <ul class="max-w-2xl mx-auto mt-8">{move || meals_data()}</ul>
        </Transition>
    }
}
