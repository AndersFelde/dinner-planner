use crate::api::receipt::{create_receipt_with_items, scan_receipt};
use crate::components::models::receipt::Receipt;
use crate::models::receipt::{ReceiptForm, ReceiptItemForm, ReceiptWithItems};
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn ReceiptForm(
    receipt: RwSignal<Option<ReceiptWithItems>>,
    completed: WriteSignal<bool>,
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
            completed.set(true)
        }
    });

    let on_submit = move |receipt_form: ReceiptForm, receipt_items_forms: Vec<ReceiptItemForm>| {
        add_receipt_action.dispatch((receipt_form, receipt_items_forms));
    };
    let on_cancel = move || completed.set(true);

    let (store, total, items) = (receipt_form.store, receipt_form.total, receipt_items_forms);

    let (store, set_store) = signal(store);
    let (total, set_total) = signal(total);
    let (items, set_items) = signal(items);

    Effect::watch(
        move || items.get(),
        move |items, _, _| {
            set_total.set(items.iter().map(|i| i.price).sum());
        },
        false,
    );

    let add_item = move |_| {
        set_items.update(|ings| {
            ings.push(ReceiptItemForm {
                receipt_id: 0,
                name: String::from(""),
                price: 0f32,
            });
        });
    };

    let form_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let receipt = ReceiptForm {
            store: store.get(),
            total: total.get(),
            datetime: receipt_form.datetime
        };
        // Call your server function to save meal and ingredients here
        on_submit(receipt, items.get());
    };

    view! {
        <div class="max-w-lg mx-auto my-6 rounded-xl border border-gray-200 bg-white p-6 shadow-md">
            // <!-- Receipt Header -->
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-xl font-semibold text-gray-900">
                    <input
                        type="text"
                        prop:value=store
                        on:input=move |ev| set_store(event_target_value(&ev))
                    />
                </h3>
                <span class="text-gray-700 font-medium">
                    {move || format!("{:.2},-", total.get())}
                </span>
            </div>

            // <!-- Date -->
            <p class="text-sm text-gray-500 mb-4">
                {format!("{}", receipt_form.datetime.format("%Y-%m-%d %H:%M"))}
            </p>

            // <!-- Items Table -->
            <form on:submit=form_submit>
                <div class="overflow-x-auto">
                    <table class="w-full text-left text-sm border-collapse">
                        <thead>
                            <tr class="border-b border-gray-200">
                                <th class="py-2 px-4 text-gray-700 font-medium">"Item"</th>
                                <th class="py-2 px-4 text-gray-700 font-medium">"Price"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                items
                                    .get()
                                    .iter()
                                    .enumerate()
                                    .map(|(i, item)| {
                                        let price = format!("{:.2}", item.price);
                                        view! {
                                            <tr class="border-b border-gray-100 hover:bg-gray-50">
                                                <td class="py-2 px-4">
                                                    <input
                                                        required
                                                        placeholder="Ingredient name"
                                                        prop:value=item.name.clone()
                                                        type="text"
                                                        on:input:target=move |ev| {
                                                            set_items
                                                                .update(|items| items[i].name = ev.target().value())
                                                        }
                                                    />
                                                </td>
                                                <td class="py-2 px-4">
                                                    <input
                                                        required
                                                        prop:value=price
                                                        type="number"
                                                        step="0.01"
                                                        on:input:target=move |ev| {
                                                            if let Ok(price) = ev.target().value().parse::<f32>() {
                                                                set_items.update(|items| items[i].price = price)
                                                            }
                                                        }
                                                    />
                                                </td>
                                            </tr>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </tbody>
                    </table>
                    <button
                        type="button"
                        on:click=add_item
                        class="w-full py-2 bg-blue-100 text-blue-700 rounded-lg hover:bg-blue-200 transition mb-2"
                    >
                        "+ Add Ingredient"
                    </button>
                    <button
                        type="submit"
                        class="w-full py-2 bg-blue-500 text-white font-semibold rounded-lg hover:bg-blue-600 transition"
                    >
                        Save receipt
                    </button>
                </div>
            </form>
        </div>
    }
}
