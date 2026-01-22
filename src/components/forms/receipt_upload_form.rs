use crate::api::receipt::scan_receipt;
use crate::components::models::receipt::Receipt;
use crate::models::receipt::ReceiptWithItems;
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn ReceiptUpload(receipt: RwSignal<Option<ReceiptWithItems>>) -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| {
        let data = data.clone();
        async move { scan_receipt(data.into()).await }
    });

    let upload_submitted = upload_action.input();
    let pending = upload_action.pending();
    let upload = upload_action.value();

    Effect::new(move || {
        if let Some(Ok(new_receipt)) = upload_action.value().get() {
            receipt.set(Some(new_receipt));
        }
    });

    view! {
        <div class="max-w-md mx-auto mt-10 rounded-2xl border border-gray-200 bg-white p-6 shadow-sm">
            <h3 class="text-xl font-semibold text-gray-900">
                Upload Receipt
            </h3>

            <p class="mt-1 text-sm text-gray-600">
                Upload a receipt image or PDF to scan and extract details.
            </p>

            <form
                class="mt-6 space-y-4"
                on:submit=move |ev: SubmitEvent| {
                    ev.prevent_default();
                    let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
                    let form_data = FormData::new_with_form(&target).unwrap();
                    upload_action.dispatch_local(form_data);
                }
            >
                <label class="block">
                    <span class="sr-only">Choose file</span>
                    <input
                        type="file"
                        name="file_to_upload"
                        class="block w-full text-sm text-gray-700
                               file:mr-4 file:rounded-md file:border-0
                               file:bg-gray-100 file:px-4 file:py-2
                               file:text-sm file:font-medium
                               file:text-gray-700
                               hover:file:bg-gray-200
                               focus:outline-none"
                    />
                </label>

                <button
                    type="submit"
                    class="inline-flex w-full items-center justify-center rounded-md
                           bg-indigo-600 px-4 py-2 text-sm font-semibold text-white
                           hover:bg-indigo-700
                           focus:outline-none focus:ring-2 focus:ring-indigo-500
                           focus:ring-offset-2
                           disabled:cursor-not-allowed disabled:opacity-50"
                    disabled=move || pending.get()
                >
                    {move || if pending.get() { "Uploading…" } else { "Upload Receipt" }}
                </button>
            </form>

            <div class="mt-4 min-h-[1.5rem] text-sm">
                {move || {


                    if upload_submitted.read().is_none() && upload.read().is_none() {
                        view! {
                            <p class="text-gray-500">
                                Select a file to begin.
                            </p>
                        }.into_any()
                    } else if pending.get() {
                        view! {
                            <p class="text-indigo-600">
                                Processing receipt
                            </p>
                        }.into_any()
                    } else if let Some(Ok(_)) = upload.read().as_ref() {
                        view! {
                            // <Receipt receipt_with_items=value/>
                            <p class="text-green-600 font-medium">
                                Success
                            </p>
                        }.into_any()
                    } else if let Some(Err(err)) = upload.read().as_ref() {
                        view! {
                            <p class="text-red-600">
                                {format!("Error: {:?}", err)}
                            </p>
                        }.into_any()
                    } else {
                        view! {
                            <p class="text-gray-500">
                                Something went wrong.
                            </p>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
