use leptos::logging::log;
use leptos::prelude::*;
use leptos::IntoView;

use crate::components::csr::js::check_notification_permission;
use crate::components::csr::js::request_notification_permission;
use crate::components::csr::js::set_badge;
use crate::components::csr::NotificationStatus;

#[component]
pub fn Notifications() -> impl IntoView {
    check_notification_permission();
    set_badge(10);
    let (permission, set_permission) = signal(false);
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
        // ...rest of your content...
    }
}
