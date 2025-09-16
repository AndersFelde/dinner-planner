use crate::api::extra_items::{get_extra_item, insert_extra_item, update_extra_item};
use crate::api::week::Week;
use crate::app::RouteUrl;
use crate::components::error_list;
use crate::models::extra_item::{ExtraItem, ExtraItemForm};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map, use_query_map};

#[component]
pub fn UpdateExtraItemForm() -> impl IntoView {
    let params = use_params_map();
    let extra_item_resource = Resource::new(
        move || {
            params
                .read()
                .get("id")
                .and_then(|id| id.parse::<i32>().ok())
        },
        move |id| async move {
            match id {
                Some(id) => get_extra_item(id).await.map(Some),
                None => Err(ServerFnError::new("No valid id provided")),
            }
        },
    );
    let add_extra_item_action = Action::new(|extra_item: &ExtraItem| {
        let extra_item = extra_item.clone();
        async move { update_extra_item(extra_item).await }
    });
    let query = use_query_map();
    let navigate = use_navigate();
    let redirect = move || {
        if let Some(url) = query.read().get("redirect") {
            navigate(&url, Default::default());
        } else {
            let w = Week::current();
            // TODO: this is a hack
            navigate(
                &format!(
                    "{}?extra-items",
                    RouteUrl::ShoppingList {
                        week: w.week,
                        year: w.year
                    }
                    .to_string()
                ),
                Default::default(),
            );
        }
    };
    Effect::new(move || {
        if let Some(Ok(_)) = add_extra_item_action.value().get() {
            redirect();
        }
    });
    // match meal_resource.get() {
    //     Some(Ok(meal)) => Either::Left({
    let extra_item_form = move || {
        extra_item_resource.get().map(|extra_item| {
            extra_item.map(|extra_item| {
                extra_item.map(|extra_item| {
                    let id = extra_item.id.clone();
                    let on_submit = move |extra_item_form: ExtraItemForm| {
                        add_extra_item_action.dispatch(ExtraItem {
                            id,
                            name: extra_item_form.name,
                            amount: extra_item_form.amount,
                            bought: extra_item_form.bought,
                        });
                    };
                    view! { <ExtraItemForm extra_item=Some(extra_item) on_submit=on_submit /> }
                })
            })
        })
    };
    view! {
        <Transition fallback=move || {
            view! { <span>"Loading..."</span> }
        }>
            <ErrorBoundary fallback=error_list>{extra_item_form}</ErrorBoundary>
        //
        </Transition>
    }
    // if let Some(Ok(meal)) = meal_resource.get() {
}
#[component]
pub fn CreateExtraItemForm() -> impl IntoView {
    let add_extra_item_action = Action::new(|extra_item: &ExtraItemForm| {
        let extra_item_form = extra_item.clone();
        async move { insert_extra_item(extra_item_form).await }
    });
    let query = use_query_map();
    let navigate = use_navigate();
    // TODO: move into meal form
    let redirect = move || {
        if let Some(url) = query.read().get("redirect") {
            navigate(&url, Default::default());
        } else {
            let w = Week::current();
            // TODO: this is a hack
            navigate(
                &format!(
                    "{}?extra-items",
                    RouteUrl::ShoppingList {
                        week: w.week,
                        year: w.year
                    }
                    .to_string()
                ),
                Default::default(),
            );
        }
    };
    Effect::new(move || {
        if let Some(Ok(_)) = add_extra_item_action.value().get() {
            redirect();
        }
    });

    let on_submit = move |extra_item_form: ExtraItemForm| {
        add_extra_item_action.dispatch(extra_item_form);
    };
    view! { <ExtraItemForm extra_item=None on_submit=on_submit /> }
}

#[component]
pub fn ExtraItemForm<A>(extra_item: Option<ExtraItem>, on_submit: A) -> impl IntoView
where
    A: Fn(ExtraItemForm) + 'static,
{
    // Signals for meal fields
    let (name, amount, bought) = if let Some(extra_item) = extra_item.clone() {
        (extra_item.name, extra_item.amount, extra_item.bought)
    } else {
        (String::new(), 1, false)
    };
    let name = RwSignal::new(name);
    let amount = RwSignal::new(amount);
    let bought = RwSignal::new(bought);

    // Add new ingredient field

    // Remove ingredient field

    // Handle form submission (pseudo-code, replace with your server call)
    let form_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // Call your server function to save meal and ingredients here
        on_submit(ExtraItemForm {
            name: name.get(),
            amount: amount.get(),
            bought: bought.get(),
        });
    };

    let action_name = {
        if extra_item.is_some() {
            "Update Extra Item"
        } else {
            "Create Extra Item"
        }
    };

    view! {
        <div class="max-w-lg mx-auto mt-8 p-6 bg-white dark:bg-gray-900 rounded-xl shadow-lg">
            <A href=RouteUrl::Home attr:class="text-blue-500 hover:underline mb-4 inline-block">
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

            </A>
            <form on:submit=form_submit class="space-y-6">
                <div class="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border mb-2 flex flex-nowrap gap-2 items-center justify-center">
                    <input
                        type="text"
                        placeholder="Name"
                        prop:value=name.get()
                        bind:value=name
                        class="px-3 py-2 flex-1 min-w-0 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-400 dark:bg-gray-700 dark:text-white"
                        required
                    />
                    // Amount display and buttons
                    <span class="flex items-center gap-2">
                        <button
                            type="button"
                            class="w-8 h-8 flex items-center justify-center rounded-full border border-gray-300 bg-white text-blue-600 text-lg font-bold shadow hover:bg-blue-100 hover:text-blue-800 transition"
                            on:click=move |_| { amount.update(|amount| { *amount -= 1 }) }
                        >
                            "-"
                        </button>
                        <span class="px-3 py-2 border rounded-lg bg-white dark:bg-gray-700 dark:text-white min-w-[2.5rem] text-center font-semibold text-lg">
                            {move || amount.get()}
                        </span>
                        <button
                            type="button"
                            class="w-8 h-8 flex items-center justify-center rounded-full border border-gray-300 bg-white text-blue-600 text-lg font-bold shadow hover:bg-blue-100 hover:text-blue-800 transition"
                            on:click=move |_| { amount.update(|amount| { *amount += 1 }) }
                        >
                            "+"
                        </button>
                    </span>
                    <label class="flex items-center gap-2 ml-4">
                        <input
                            type="checkbox"
                            prop:checked=bought.get()
                            bind:checked=bought
                            class="form-checkbox h-5 w-5 text-blue-600 rounded border-gray-300 focus:ring-blue-400"
                        />
                        <span class="text-gray-700 dark:text-gray-200">Bought</span>
                    </label>
                </div>
                <button
                    type="submit"
                    class="w-full py-2 bg-blue-500 text-white font-semibold rounded-lg hover:bg-blue-600 transition"
                >
                    {action_name}
                </button>
            </form>
        </div>
    }
}
