pub mod forms;
pub mod models;
pub mod csr;
pub mod notifications;
pub mod modal;
pub mod buttons;
pub mod toasts;

use leptos::prelude::*;
use std::collections::HashSet;

use crate::components::toasts::ToastStore;

pub fn error_list(errors: ArcRwSignal<Errors>) -> impl IntoView {
    let toast_store = expect_context::<ToastStore>();
    let displayed = RwSignal::new(HashSet::<String>::new());

    Effect::new(move || {
        let entries = errors.with(|errors| {
            errors
                .iter()
                .map(|(id, e)| (format!("{id:?}"), e.to_string()))
                .collect::<Vec<_>>()
        });

        if entries.is_empty() {
            return;
        }

        displayed.update(|seen| {
            for (id, message) in entries {
                if seen.insert(id) {
                    toast_store.push_error(message);
                }
            }
        });
    });

    view! {}
}
