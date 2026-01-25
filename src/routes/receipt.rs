use crate::{
    app::RouteUrl,
    components::{
        forms::receipt_form::ReceiptForm, forms::receipt_upload_form::ReceiptUpload,
        models::receipt::Receipt,
    },
    models::receipt::{ReceiptForm, ReceiptItemForm, ReceiptWithItems},
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ReceiptRoute() -> impl IntoView {
    let receipt: RwSignal<Option<ReceiptWithItems>> = RwSignal::new(None);
    let receipt_completed: RwSignal<bool> = RwSignal::new(false);
    let receipt_form: RwSignal<Option<ReceiptForm>> = RwSignal::new(None);
    let receipt_items_forms: RwSignal<Option<Vec<ReceiptItemForm>>> = RwSignal::new(None);
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
            <ReceiptUpload receipt_form receipt_items_forms />
        </div>
        <Show
            when=move || { !receipt_completed.get() && (receipt_form.read().is_some() && receipt_items_forms.read().is_some()) }
            fallback=|| view! {}
        >
            {move || {
                view! {
                    <ReceiptForm
                        receipt
                        completed=receipt_completed.write_only()
                        receipt_form=receipt_form.read().as_ref().unwrap().clone()
                        receipt_items_forms=receipt_items_forms.read().as_ref().unwrap().clone()
                    />
                }
            }}

        </Show>

        <Show
            when=move || { receipt.read().is_some() }
            fallback=|| view! {}
        >
            {move || {
                let receipt = receipt.get().unwrap();
                view! {
                    <Receipt
                        receipt_with_items = &receipt
                    />
                }
            }}

        </Show>
    }
}
