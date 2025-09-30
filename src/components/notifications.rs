use chrono::Datelike;
use chrono::Local;
use leptos::prelude::*;
use leptos::IntoView;
use reactive_stores::Store;
use web_sys::NotificationPermission;

use crate::api::extra_items::get_extra_items_not_bought;
use crate::api::week::days_for_week;
use crate::api::week::Week;
use crate::app::GlobalState;
use crate::app::GlobalStateStoreFields;
use crate::components::csr::js::check_notification_permission;
use crate::components::csr::js::request_notification_permission;
use crate::components::csr::js::set_badge;
use crate::components::csr::NotificationStatus;

#[component]
pub fn Notifications() -> impl IntoView {
    let has_permissions = RwSignal::new(true);
    // This is a hack to avoid hydration bugs because server cant check permissions beforehand
    Effect::new(move |_| match check_notification_permission() {
        NotificationStatus::Granted | NotificationStatus::NotAvailable => has_permissions.set(true),
        _ => has_permissions.set(false),
    });
    let state = expect_context::<Store<GlobalState>>();
    let extra_items_count = state.extra_items_count();
    let week_ingredients_count = state.week_ingredients_count();
    let now = Local::now().date_naive().iso_week();
    let days_resource = OnceResource::new(days_for_week(Week {
        week: now.week(),
        year: now.year(),
    }));

    let extra_items_resource = OnceResource::new(get_extra_items_not_bought());
    Effect::watch(
        move || (extra_items_count.get(), week_ingredients_count.get()),
        |items_count, _, _| set_badge(items_count.0 + items_count.1),
        true,
    );
    Effect::watch(
        move || extra_items_resource.get(),
        move |extra_items, _, _| {
            if let Some(Ok(extra_items)) = extra_items {
                extra_items_count.set(extra_items.len())
            }
        },
        true,
    );

    Effect::watch(
        move || days_resource.get(),
        move |days, _, _| {
            if let Some(Ok(days)) = days {
                week_ingredients_count.set(
                    days.iter()
                        .filter(|day| {
                            if let Some((_, ingredients)) = day.meal.as_ref() {
                                if ingredients.iter().any(|i| i.bought == false) {
                                    return true;
                                }
                            }
                            false
                        })
                        .count(),
                );
            }
        },
        true,
    );
    let on_perm_request = move |perm| match perm {
        NotificationPermission::Granted => has_permissions.set(true),
        _ => {}
    };
    view! {
        <Show when=move || !has_permissions.get() fallback=|| view! {}>
            <button
                class="fixed left-1/2 bottom-8 transform -translate-x-1/2 px-6 py-3 bg-yellow-500 text-white rounded-lg shadow-lg transition z-50"
                on:click=move |_| {
                    request_notification_permission(on_perm_request);
                }
            >
                Enable notifications
            </button>
        </Show>
    }
}
