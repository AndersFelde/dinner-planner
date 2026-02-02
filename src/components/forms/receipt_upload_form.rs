use crate::api::receipt::scan_receipt;
use crate::components::models::receipt::Receipt;
use crate::models::receipt::{ReceiptForm, ReceiptItemForm, ReceiptWithItems};
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn ReceiptUpload(
    receipt_form: RwSignal<Option<ReceiptForm>>,
    receipt_items_forms: RwSignal<Option<Vec<ReceiptItemForm>>>,
    receipt_editing: WriteSignal<bool>,
) -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| {
        let data = data.clone();
        async move { scan_receipt(data.into()).await }
    });

    let upload_submitted = upload_action.input();
    let pending = upload_action.pending();
    let upload = upload_action.value();

    let progress = RwSignal::new(0.0);

    Effect::new(move || {
        if let Some(Ok((new_receipt_form, new_items_forms))) = upload_action.value().get() {
            receipt_form.set(Some(new_receipt_form));
            receipt_items_forms.set(Some(new_items_forms));
            receipt_editing.set(true);
        }
    });

    Effect::new(move || {
        if pending.get() {
            progress.set(0.0);
            let duration_ms = 25000.0; // 24 seconds
            let interval_ms = 100; // Update every 100ms
            let increment = 100.0 / (duration_ms / interval_ms as f64);

            let interval = set_interval_with_handle(
                move || {
                    let current = progress.get();
                    if current < 100.0 {
                        progress.set((current + increment).min(100.0));
                    }
                },
                std::time::Duration::from_millis(interval_ms),
            ).ok();

            on_cleanup(move || {
                if let Some(handle) = interval {
                    handle.clear();
                }
            });
        } else {
            progress.set(0.0);
        }
    });

    view! {
        <div class="max-w-md mx-auto mt-10 rounded-2xl border border-gray-200 bg-white p-6 shadow-sm">
            <h3 class="text-xl font-semibold text-gray-900">Upload Receipt</h3>

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
                        required
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
                    bg-blue-500 px-4 py-2 text-sm font-semibold text-white
                    hover:bg-blue-700
                    focus:outline-none focus:ring-2 focus:ring-blue-400
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

                        view! { <p class="text-gray-500">Select a file to begin.</p> }
                            .into_any()
                    } else if pending.get() {
                        view! {
                            <div>
                                <p class="text-blue-600 mb-2">Processing receipt</p>
                                <div class="w-full bg-gray-200 rounded-full h-2.5">
                                    <div
                                        class="bg-blue-600 h-2.5 rounded-full transition-all duration-100"
                                        style:width=move || format!("{}%", progress.get())
                                    ></div>
                                </div>
                                <p class="text-xs text-gray-500 mt-1">
                                    {move || format!("{}%", progress.get() as u32)}
                                </p>
                            </div>
                        }.into_any()
                    } else if let Some(Ok(_)) = upload.read().as_ref() {
                        view! {
                            // <Receipt receipt_with_items=value/>
                            <p class="text-green-600 font-medium">Success</p>
                        }
                            .into_any()
                    } else if let Some(Err(err)) = upload.read().as_ref() {
                        view! { <p class="text-red-600">{format!("Error: {:?}", err)}</p> }
                            .into_any()
                    } else {
                        view! { <p class="text-gray-500">Something went wrong.</p> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
