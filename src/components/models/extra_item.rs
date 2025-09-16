use crate::api::extra_items::update_extra_item;
use crate::models::extra_item::ExtraItem;
use leptos::prelude::*;

#[component]
pub fn ExtraItem(extra_item: ExtraItem) -> impl IntoView {
    let (bought, set_bought) = signal(extra_item.bought);
    let extra_item_clone = extra_item.clone();

    let update_extra_item_action = Action::new(move |bought: &bool| {
        let mut extra_item = extra_item_clone.clone();
        let bought = bought.clone();
        async move {
            extra_item.bought = bought;
            update_extra_item(extra_item).await
        }
    });
    let on_click = move |_| {
        set_bought.update(|b| *b = !*b);
        update_extra_item_action.dispatch(bought.get());
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
            {extra_item.name.clone()}
            <Show when=move || { extra_item.amount > 1 } fallback=|| view! {}>
                <span class="ml-2 text-xs font-normal text-gray-800 bg-gray-200 dark:bg-gray-700 dark:text-gray-100 px-2 py-1 rounded">
                    {extra_item.amount.clone()}
                </span>
            </Show>

        </span>
    }
}
