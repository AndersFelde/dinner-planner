use crate::models::days_ingredients::DayIngredient;
use crate::{
    api::days_ingredients::upsert_day_ingredient, models::days_ingredients::IngredientWithBought,
};
use leptos::prelude::*;

#[component]
pub fn DayIngredient(day_ingredient: IngredientWithBought) -> impl IntoView {
    let ingredient = day_ingredient.ingredient;
    let (bought, set_bought) = signal(day_ingredient.bought);

    let day_id = day_ingredient.day_id;
    let ingredient_id = ingredient.id;
    let update_ingredient_action = Action::new(move |bought: &bool| {
        let bought = bought.clone();
        async move {
            upsert_day_ingredient(DayIngredient {
                day_id,
                ingredient_id,
                bought: bought,
            })
            .await
        }
    });
    let on_click = move |_| {
        set_bought.update(|b| *b = !*b);
        update_ingredient_action.dispatch(bought.get());
    };

    view! {
        <span
            class=move || {
                if bought.get() {
                    "inline-block px-3 py-1 m-1 rounded-full bg-green-200 text-green-900 border border-green-400 shadow-sm text-sm font-medium line-through"
                } else {
                    "inline-block px-3 py-1 m-1 rounded-full bg-red-100 text-red-900 border border-red-300 shadow-sm text-sm font-medium"
                }
            }
            on:click=on_click
        >
            {ingredient.name.clone()}
                <Show
                when= move || {ingredient.amount > 1}
                fallback = || view!{}>
                        <span class="ml-2 text-xs font-normal text-gray-800 bg-gray-200 dark:bg-gray-700 dark:text-gray-100 px-2 py-1 rounded">
                            {ingredient.amount.clone()}
                        </span>
                </Show>

        </span>
    }
}
