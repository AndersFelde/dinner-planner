use leptos::prelude::*;
use web_sys::window;

use crate::models::receipt::ReceiptWithItems;

#[component]
pub fn Receipt (receipt_with_items: ReceiptWithItems) -> impl IntoView {
    let total = receipt_with_items.total();
    let anders_sum = receipt_with_items.anders_sum();
    let andreas_sum = receipt_with_items.andreas_sum();
    let ac_sum = receipt_with_items.ac_sum();
    let receipt = receipt_with_items.receipt;
    let items = receipt_with_items.items;

    view! {
        <div class="max-w-lg mx-auto my-6 rounded-xl border border-gray-200 bg-white p-6 shadow-md">
            // <!-- Receipt Header -->
            <div class="mb-4">
                <div class="flex justify-between items-center mb-2">
                    <h3 class="text-lg font-semibold text-gray-900">{receipt.store.clone()}</h3>
                    <span class="text-sm text-gray-500">
                        {format!("{}", receipt.datetime.format("%Y-%m-%d %H:%M"))}
                    </span>
                </div>

                <div class="grid grid-cols-3 gap-2 mb-2">
                    <div
                        class="bg-blue-50 rounded-lg p-3 text-center border border-blue-200 cursor-pointer hover:bg-blue-100 transition active:scale-95"
                        on:click=move |_| {
                            let value = format!("{:.2}", anders_sum);
                            if let Some(win) = window() {
                                let _ = win.navigator().clipboard().write_text(&value);
                            }
                        }
                    >
                        <div class="text-2xl font-bold text-blue-900">{format!("{:.2}", anders_sum)}</div>
                        <div class="text-xs font-semibold text-blue-700">"Anders"</div>
                    </div>
                    <div
                        class="bg-green-50 rounded-lg p-3 text-center border border-green-200 cursor-pointer hover:bg-green-100 transition active:scale-95"
                        on:click=move |_| {
                            let value = format!("{:.2}", andreas_sum);
                            if let Some(win) = window() {
                                let _ = win.navigator().clipboard().write_text(&value);
                            }
                        }
                    >
                        <div class="text-2xl font-bold text-green-900">{format!("{:.2}", andreas_sum)}</div>
                        <div class="text-xs font-semibold text-green-700">"Andreas"</div>
                    </div>
                    <div
                        class="bg-purple-50 rounded-lg p-3 text-center border border-purple-200 cursor-pointer hover:bg-purple-100 transition active:scale-95"
                        on:click=move |_| {
                            let value = format!("{:.2}", ac_sum);
                            if let Some(win) = window() {
                                let _ = win.navigator().clipboard().write_text(&value);
                            }
                        }
                    >
                        <div class="text-2xl font-bold text-purple-900">{format!("{:.2}", ac_sum)}</div>
                        <div class="text-xs font-semibold text-purple-700">"AC"</div>
                    </div>
                </div>

                <div class="text-center text-sm text-gray-500 pb-2 border-b">
                    "Total: "
                    <span class="font-semibold text-gray-700">{format!("{:.2},-", total)}</span>
                </div>
            </div>

            // <!-- Items List -->
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
                {items
                    .iter()
                    .map(|item| {
                        let name = item.name.clone();
                        let price = format!("{:.2}", item.price);
                        let anders = item.anders_pay;
                        let andreas = item.andreas_pay;
                        let ac = item.ac_pay;
                        view! {
                            <div
                                class="grid gap-1 items-center border-b border-gray-200 py-1"
                                style="grid-template-columns: 1fr 70px 45px 45px 45px;"
                            >
                                <span class="px-1 py-1 text-sm text-gray-900 truncate">{name}</span>
                                <span class="px-1 py-1 text-sm text-gray-900">{price}</span>
                                <span class="text-center text-sm text-gray-700">{if anders { "X" } else { "" }}</span>
                                <span class="text-center text-sm text-gray-700">{if andreas { "X" } else { "" }}</span>
                                <span class="text-center text-sm text-gray-700">{if ac { "X" } else { "" }}</span>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}
