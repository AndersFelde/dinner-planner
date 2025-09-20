use leptos::logging::log;
use leptos::prelude::*;
use leptos::IntoView;
use reactive_stores::Store;

use crate::app::GlobalState;
use crate::app::GlobalStateStoreFields;
use crate::components::csr::js::check_notification_permission;
use crate::components::csr::js::request_notification_permission;
use crate::components::csr::js::set_badge;
use crate::components::csr::NotificationStatus;

#[component]
pub fn Notifications() -> impl IntoView {
    // TODO: make it initially set the badge count based on live data
    let (permission, set_permission) = signal(false);
    let state = expect_context::<Store<GlobalState>>();
    Effect::watch(
        move || {
            (
                state.extra_items_count().get(),
                state.week_ingredients_count().get(),
            )
        },
        |items_count, _, _| set_badge(items_count.0 + items_count.1),
        true,
    );
    // TODO: make this work
    Effect::new(move |_| match check_notification_permission() {
        NotificationStatus::Granted | NotificationStatus::NotAvailable => {
            log!("Granted or not available");
            set_permission.set(false)
        }
        _ => {
            log!("Can enable notifications");
            set_permission.set(true)
        }
    });
    view! {
        <Show when=move || permission.get() fallback=|| view! {}>
            <button
                class="fixed left-1/2 bottom-8 transform -translate-x-1/2 px-6 py-3 bg-yellow-500 text-white rounded-lg shadow-lg transition z-50"
                on:click=|_| {
                    request_notification_permission();
                }
            >
                Enable notifications
            </button>
        </Show>
    }
}
