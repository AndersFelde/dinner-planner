use crate::api::meal::get_all_meals_with_ingredients;
use crate::app::RouteUrl;
use crate::components::error_list;
use crate::components::forms::meal_form::CreateMealForm;
use crate::components::modal::Modal;
use crate::components::models::meal::Meal;
use crate::models::meal::MealWithIngredients;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_use::math::use_not;

#[component]
pub fn MealList() -> impl IntoView {
    let meals_resource = OnceResource::new(get_all_meals_with_ingredients());
    let create_completed = RwSignal::new(true);
    let show_create = use_not(create_completed);
    let new_meal: RwSignal<Option<MealWithIngredients>> = RwSignal::new(None);
    let meals: RwSignal<Vec<MealWithIngredients>> = RwSignal::new(Vec::new());
    let search_input = RwSignal::new(String::new());

    Effect::watch(
        move || meals_resource.get(),
        move |r_meals, _, _| {
            if let Some(Ok(r_meals)) = r_meals {
                meals.set(r_meals.clone());
            }
        },
        true,
    );
    Effect::watch(
        move || new_meal.get(),
        move |new_meal, _, _| {
            if let Some(new_meal) = new_meal {
                meals.write().push(new_meal.clone())
            }
        },
        false,
    );
    // Close modal when create_completed becomes true
    let meals_data = move || {
        let meals_vec = meals.get();
        let search_input = search_input.get();
        let filtered = if !search_input.is_empty() {
            let search_input = search_input.to_lowercase();
            meals_vec
                .iter()
                .filter(|m| m.meal.name.to_lowercase().contains(&search_input))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            meals_vec.clone()
        };
        filtered
            .into_iter()
            .map(|meal| {
                view! { <Meal meal=meal /> }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <Modal show=show_create>
            <CreateMealForm meal=new_meal completed=create_completed.write_only() />
        </Modal>
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
                create_completed.set(false);
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
        <div class="flex justify-center items-center gap-4 mb-2 sticky top-0 z-10 bg-white dark:bg-gray-800 py-2 shadow">
            <span class="font-bold text-base text-gray-900 dark:text-white">"Meals"</span>
        </div>

        <input
            type="text"
            placeholder="Search"
            prop:value=search_input
            bind:value=search_input
            class="px-3 py-2 flex-1 min-w-0 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-700 dark:text-white"
            required
        />
        <Transition fallback=move || {
            view! { <p class="text-center text-gray-400 dark:text-gray-800">"Loading..."</p> }
        }>
            <ErrorBoundary fallback=error_list>
                <div class="flex flex-col gap-4 py-2 items-center justify-center">
                    {move || meals_data()}
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
