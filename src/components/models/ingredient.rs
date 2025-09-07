use crate::models::days_ingredients::DayIngredient;
use crate::{
    api::days_ingredients::upsert_day_ingredient, models::days_ingredients::IngredientWithBought,
};
use leptos::prelude::*;

#[component]
pub fn DayIngredient(day_ingredient: IngredientWithBought) -> impl IntoView {
    let ingredient = day_ingredient.ingredient;
    let (bought, set_bought) = signal(day_ingredient.bought);

    let label = if ingredient.amount > 1 {
        format!("{} x{}", ingredient.name, ingredient.amount)
    } else {
        ingredient.name.clone()
    };
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
                    "inline-block px-3 py-1 m-1 rounded-full bg-green-200 text-green-900 border border-green-400 shadow-sm text-sm font-medium"
                } else {
                    "inline-block px-3 py-1 m-1 rounded-full bg-red-100 text-red-900 border border-red-300 shadow-sm text-sm font-medium"
                }
            }
            on:click=on_click
        >
            {label}
        </span>
    }
}
