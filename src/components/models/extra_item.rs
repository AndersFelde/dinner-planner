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
                format!(
                    "flex flex-row items-center justify-center w-full max-w-sm min-h-[60px] px-6 py-3 m-3 rounded-xl border-2 shadow text-base font-semibold transition-colors duration-200 cursor-pointer space-x-4 {}",
                    if bought.get() {
                        "bg-green-200 text-green-900 border-green-400 line-through"
                    } else {
                        "bg-red-100 text-red-900 border-red-300"
                    },
                )
            }
            style="box-sizing: border-box;"
            on:click=on_click
        >
            <span class="text-center">{extra_item.name.clone()}</span>
            <span class="text-sm font-normal text-gray-800 bg-gray-200 dark:bg-gray-700 dark:text-gray-100 px-3 py-1 rounded flex-shrink-0 flex items-center justify-center">
                {extra_item.amount.clone()}
            </span>
        </span>
    }
}