pub mod forms;
pub mod models;
pub mod csr;
pub mod notifications;
pub mod modal;

use leptos::prelude::*;

pub fn error_list(errors: ArcRwSignal<Errors>) -> impl IntoView {
    let error_list = move || {
        errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! {
                        <li class="flex items-center gap-2 mb-2 list-none">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-5 text-red-500"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"
                                />
                            </svg>
                            <span class="text-sm">{e.to_string()}</span>
                        </li>
                    })
                    .collect::<Vec<_>>()
            })
    };

    view! {
        <div
            class="absolute top-4 right-4 z-50 bg-red-100 border border-red-400 text-red-700 px-6 py-4 rounded shadow-lg"
            style="min-width: 250px; max-width: 400px;"
        >
            <h2 class="font-bold mb-2">"Error"</h2>
            <ul class="p-0 m-0">{error_list}</ul>
        </div>
    }
}
