use crate::{app::RouteUrl, components::forms::receipt_upload_form::ReceiptUpload};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Receipt() -> impl IntoView {
    view! {
        <A href=RouteUrl::Home.to_string()>
            <button
                type="button"
                class="fixed bottom-4 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg  focus:outline-none focus:ring-2  transition flex items-center justify-center whitespace-nowrap"
                title="View shopping list"
            >
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

            </button>
        </A>

        <div class="w-80 mx-auto">
            <ReceiptUpload />
        </div>
    }
}
