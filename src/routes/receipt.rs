use crate::{
    api::receipt::get_all_receipts_with_items,
    app::RouteUrl,
    components::{
        error_list,
        forms::{receipt_form::ReceiptForm, receipt_upload_form::ReceiptUpload},
        models::receipt::Receipt,
    },
    models::receipt::{ReceiptForm, ReceiptItemForm, ReceiptWithItems},
};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ReceiptCreateRoute() -> impl IntoView {
    let receipt: RwSignal<Option<ReceiptWithItems>> = RwSignal::new(None);
    let receipt_editing: RwSignal<bool> = RwSignal::new(false);
    let receipt_form: RwSignal<Option<ReceiptForm>> = RwSignal::new(None);
    let receipt_items_forms: RwSignal<Option<Vec<ReceiptItemForm>>> = RwSignal::new(None);
    view! {
        <A href=RouteUrl::ReceiptList.to_string()>
            <button
                type="button"
                class="fixed bottom-19 right-4 z-50 px-4 py-3 rounded-full bg-blue-500 text-white font-semibold text-base shadow-lg  focus:outline-none focus:ring-2  transition flex items-center justify-center whitespace-nowrap"
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
                        d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z"
                    />
                </svg>
            </button>
        </A>
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

        <Show
            when=move || { !receipt_editing.get() && receipt.get().is_none() }
            fallback=|| view! {}
        >
            {move || {
                view! {
                    <div class="w-80 mx-auto">
                        <ReceiptUpload
                            receipt_form
                            receipt_items_forms
                            receipt_editing=receipt_editing.write_only()
                        />
                    </div>
                }
            }}

        </Show>
        <Show
            when=move || {
                receipt_editing.get() && receipt_form.read().is_some()
                    && receipt_items_forms.read().is_some()
            }
            fallback=|| view! {}
        >
            {move || {
                view! {
                    <ReceiptForm
                        receipt
                        receipt_editing=receipt_editing.write_only()
                        receipt_form=receipt_form.read().as_ref().unwrap().clone()
                        receipt_items_forms=receipt_items_forms.read().as_ref().unwrap().clone()
                    />
                }
            }}

        </Show>

        <Show when=move || { receipt.read().is_some() } fallback=|| view! {}>
            {move || {
                // let receipt = receipt.get().unwrap();
                view! { <Receipt receipt_with_items=receipt.get().unwrap() /> }
            }}

        </Show>
    }
}

#[component]
pub fn ReceiptListRoute() -> impl IntoView {
    let receipt_resource = OnceResource::new(get_all_receipts_with_items());
    let receipts: RwSignal<Vec<ReceiptWithItems>> = RwSignal::new(Vec::new());

    Effect::watch(
        move || receipt_resource.get(),
        move |r_receipts, _, _| {
            if let Some(Ok(r_receipts)) = r_receipts {
                receipts.set(r_receipts.clone());
            }
        },
        true,
    );
    let receipts_data = move || {
        receipts
            .get()
            .into_iter()
            .map(|receipt_with_items| {
                view! { <Receipt receipt_with_items /> }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <A href=RouteUrl::ReceiptCreate.to_string()>
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
        <Transition fallback=move || {
            view! { <p class="text-center text-gray-400 dark:text-gray-800">"Loading..."</p> }
        }>
            <ErrorBoundary fallback=error_list>
                <div class="flex flex-col gap-4 py-2 items-center justify-center">
                    {move || receipts_data()}
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
