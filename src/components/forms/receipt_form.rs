use crate::api::receipt::{create_receipt_with_items, scan_receipt};
use crate::components::models::receipt::Receipt;
use crate::models::receipt::{ReceiptForm, ReceiptItemForm, ReceiptWithItems};
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn ReceiptForm(
    receipt: RwSignal<Option<ReceiptWithItems>>,
    receipt_editing: WriteSignal<bool>,
    receipt_form: ReceiptForm,
    receipt_items_forms: Vec<ReceiptItemForm>,
) -> impl IntoView {
    let add_receipt_action = Action::new(|input: &(ReceiptForm, Vec<ReceiptItemForm>)| {
        let receipt_form = input.0.clone();
        let receipt_items_forms = input.1.clone();
        async move { create_receipt_with_items(receipt_form, receipt_items_forms).await }
    });

    Effect::new(move || {
        if let Some(Ok(new_receipt)) = add_receipt_action.value().get() {
            receipt.set(Some(new_receipt));
            receipt_editing.set(false)
        }
    });

    let on_submit = move |receipt_form: ReceiptForm, receipt_items_forms: Vec<ReceiptItemForm>| {
        add_receipt_action.dispatch((receipt_form, receipt_items_forms));
    };
    let on_cancel = move || receipt_editing.set(false);

    let total = receipt_items_forms.iter().map(|i| i.price).sum::<f32>();
    let anders_total = receipt_items_forms
        .iter()
        .filter(|i| i.anders_pay)
        .map(|i| i.price / (i.anders_pay as u8 + i.andreas_pay as u8 + i.ac_pay as u8) as f32)
        .sum::<f32>();
    let andreas_total = receipt_items_forms
        .iter()
        .filter(|i| i.andreas_pay)
        .map(|i| i.price / (i.anders_pay as u8 + i.andreas_pay as u8 + i.ac_pay as u8) as f32)
        .sum::<f32>();
    let ac_total = receipt_items_forms
        .iter()
        .filter(|i| i.ac_pay)
        .map(|i| i.price / (i.anders_pay as u8 + i.andreas_pay as u8 + i.ac_pay as u8) as f32)
        .sum::<f32>();

    let (store, set_store) = signal(receipt_form.store);
    let (total, set_total) = signal(total);
    let (anders_total, set_anders_total) = signal(anders_total);
    let (andreas_total, set_andreas_total) = signal(andreas_total);
    let (ac_total, set_ac_total) = signal(ac_total);
    let (items, set_items) = signal(receipt_items_forms);

    Effect::watch(
        move || items.get(),
        move |items, _, _| {
            set_total.set(items.iter().map(|i| i.price).sum());
            set_anders_total.set(
                items
                    .iter()
                    .filter(|i| i.anders_pay)
                    .map(|i| {
                        i.price / (i.anders_pay as u8 + i.andreas_pay as u8 + i.ac_pay as u8) as f32
                    })
                    .sum(),
            );
            set_andreas_total.set(
                items
                    .iter()
                    .filter(|i| i.andreas_pay)
                    .map(|i| {
                        i.price / (i.anders_pay as u8 + i.andreas_pay as u8 + i.ac_pay as u8) as f32
                    })
                    .sum(),
            );
            set_ac_total.set(
                items
                    .iter()
                    .filter(|i| i.ac_pay)
                    .map(|i| {
                        i.price / (i.anders_pay as u8 + i.andreas_pay as u8 + i.ac_pay as u8) as f32
                    })
                    .sum(),
            );
        },
        false,
    );

    let add_item = move |_| {
        set_items.update(|ings| {
            ings.push(ReceiptItemForm {
                receipt_id: 0,
                name: String::from(""),
                price: 0f32,
                andreas_pay: true,
                anders_pay: true,
                ac_pay: true,
            });
        });
    };

    let form_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let receipt = ReceiptForm {
            store: store.get(),
            datetime: receipt_form.datetime,
        };
        // Call your server function to save meal and ingredients here
        on_submit(receipt, items.get());
    };

    view! {
        <div class="max-w-lg mx-auto my-6 rounded-xl border border-gray-200 bg-white p-6 shadow-md">
            // <!-- Receipt Header -->
            <div class="mb-4">
                <div class="flex justify-between items-center mb-2">
                    <input
                        type="text"
                        prop:value=store
                        placeholder="Store name"
                        class="text-lg font-semibold text-gray-900 border-b border-gray-300 focus:border-blue-500 focus:outline-none bg-transparent"
                        on:input=move |ev| set_store(event_target_value(&ev))
                    />
                    <span class="text-sm text-gray-500">
                        {format!("{}", receipt_form.datetime.format("%Y-%m-%d %H:%M"))}
                    </span>
                </div>

                <div class="grid grid-cols-3 gap-2 mb-2">
                    <div
                        class="bg-blue-50 rounded-lg p-3 text-center border border-blue-200 cursor-pointer hover:bg-blue-100 transition active:scale-95"
                        on:click=move |_| {
                            let value = format!("{:.2}", anders_total.get());
                            if let Some(window) = web_sys::window() {
                                let _ = window.navigator().clipboard().write_text(&value);
                            }
                        }
                    >
                        <div class="text-2xl font-bold text-blue-900">
                            {move || format!("{:.2}", anders_total.get())}
                        </div>
                        <div class="text-xs font-semibold text-blue-700">"Anders"</div>
                    </div>
                    <div
                        class="bg-green-50 rounded-lg p-3 text-center border border-green-200 cursor-pointer hover:bg-green-100 transition active:scale-95"
                        on:click=move |_| {
                            let value = format!("{:.2}", andreas_total.get());
                            if let Some(window) = web_sys::window() {
                                let _ = window.navigator().clipboard().write_text(&value);
                            }
                        }
                    >
                        <div class="text-2xl font-bold text-green-900">
                            {move || format!("{:.2}", andreas_total.get())}
                        </div>
                        <div class="text-xs font-semibold text-green-700">"Andreas"</div>
                    </div>
                    <div
                        class="bg-purple-50 rounded-lg p-3 text-center border border-purple-200 cursor-pointer hover:bg-purple-100 transition active:scale-95"
                        on:click=move |_| {
                            let value = format!("{:.2}", ac_total.get());
                            if let Some(window) = web_sys::window() {
                                let _ = window.navigator().clipboard().write_text(&value);
                            }
                        }
                    >
                        <div class="text-2xl font-bold text-purple-900">
                            {move || format!("{:.2}", ac_total.get())}
                        </div>
                        <div class="text-xs font-semibold text-purple-700">"AC"</div>
                    </div>
                </div>

                <div class="text-center text-sm text-gray-500 pb-2 border-b">
                    "Total: "
                    <span class="font-semibold text-gray-700">
                        {move || format!("{:.2},-", total.get())}
                    </span>
                </div>
            </div>

            // <!-- Items List -->
            <form on:submit=form_submit class="space-y-2">
                <div class="space-y-1">
                    <div
                        class="grid gap-1 text-xs font-semibold text-gray-600 mb-1"
                        style="grid-template-columns: 1fr 70px 45px 45px 45px;"
                    >
                        <div>"Item"</div>
                        <div>"Price"</div>
                        <div class="text-center">"An"</div>
                        <div class="text-center">"As"</div>
                        <div class="text-center">"AC"</div>
                    </div>
                    {move || {
                        items
                            .get()
                            .iter()
                            .enumerate()
                            .map(|(i, item)| {
                                let price = format!("{:.2}", item.price);
                                view! {
                                    <div
                                        class="grid gap-1 items-center border-b border-gray-200 py-1"
                                        style="grid-template-columns: 1fr 70px 45px 45px 45px;"
                                    >
                                        <input
                                            required
                                            placeholder="Item"
                                            prop:value=item.name.clone()
                                            type="text"
                                            class="px-1 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500 min-w-0"
                                            on:input:target=move |ev| {
                                                set_items
                                                    .update(|items| items[i].name = ev.target().value())
                                            }
                                        />
                                        <input
                                            required
                                            value=price
                                            type="number"
                                            step="0.01"
                                            class="px-1 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500 w-full"
                                            on:input:target=move |ev| {
                                                if let Ok(price) = ev.target().value().parse::<f32>() {
                                                    set_items.update(|items| items[i].price = price)
                                                }
                                            }
                                        />
                                        <input
                                            type="checkbox"
                                            checked=item.anders_pay
                                            class="w-4 h-4 cursor-pointer mx-auto"
                                            on:input:target=move |ev| {
                                                set_items
                                                    .update(|items| items[i].anders_pay = ev.target().checked())
                                            }
                                        />
                                        <input
                                            type="checkbox"
                                            checked=item.andreas_pay
                                            class="w-4 h-4 cursor-pointer mx-auto"
                                            on:input:target=move |ev| {
                                                set_items
                                                    .update(|items| {
                                                        items[i].andreas_pay = ev.target().checked();
                                                    })
                                            }
                                        />
                                        <input
                                            type="checkbox"
                                            checked=item.ac_pay
                                            class="w-4 h-4 cursor-pointer mx-auto"
                                            on:input:target=move |ev| {
                                                set_items
                                                    .update(|items| items[i].ac_pay = ev.target().checked())
                                            }
                                        />
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>

                <div class="space-y-1 pt-2">
                    <button
                        type="button"
                        on:click=add_item
                        class="w-full py-1 text-sm bg-blue-100 text-blue-700 rounded hover:bg-blue-200 transition font-medium"
                    >
                        "+ Add Item"
                    </button>
                    <button
                        type="submit"
                        class="w-full py-1 text-sm bg-blue-500 text-white font-semibold rounded hover:bg-blue-600 transition"
                    >
                        "Save Receipt"
                    </button>
                    <button
                        type="button"
                        class="w-full py-1 text-sm bg-red-500 text-white font-semibold rounded hover:bg-red-600 transition"
                        on:click=move |_| on_cancel()
                    >
                        "Cancel"
                    </button>
                </div>
            </form>
        </div>
    }
}
