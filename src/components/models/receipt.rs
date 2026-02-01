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
    let days = receipt_with_items.days;
    
    let copied_signal = RwSignal::new(None::<&str>);
    
    let copy_to_clipboard = move |name: &'static str, value: f32| {
        let value_str = format!("{:.2}", value);
        if let Some(win) = window() {
            let _ = win.navigator().clipboard().write_text(&value_str);
            copied_signal.set(Some(name));
            set_timeout(move || copied_signal.set(None), std::time::Duration::from_millis(1500));
        }
    };

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
                        class=move || {
                            if copied_signal.get() == Some("anders") {
                                "bg-blue-200 rounded-lg p-3 text-center border-2 border-blue-400 cursor-pointer transition"
                            } else {
                                "bg-blue-50 rounded-lg p-3 text-center border border-blue-200 cursor-pointer hover:bg-blue-100 transition active:scale-95"
                            }
                        }
                        on:click=move |_| copy_to_clipboard("anders", anders_sum)
                    >
                        <div class="text-2xl font-bold text-blue-900">{format!("{:.2}", anders_sum)}</div>
                        <div class="text-xs font-semibold text-blue-700">
                            {move || if copied_signal.get() == Some("anders") { "Copied!" } else { "Anders" }}
                        </div>
                    </div>
                    <div
                        class=move || {
                            if copied_signal.get() == Some("andreas") {
                                "bg-green-200 rounded-lg p-3 text-center border-2 border-green-400 cursor-pointer transition"
                            } else {
                                "bg-green-50 rounded-lg p-3 text-center border border-green-200 cursor-pointer hover:bg-green-100 transition active:scale-95"
                            }
                        }
                        on:click=move |_| copy_to_clipboard("andreas", andreas_sum)
                    >
                        <div class="text-2xl font-bold text-green-900">{format!("{:.2}", andreas_sum)}</div>
                        <div class="text-xs font-semibold text-green-700">
                            {move || if copied_signal.get() == Some("andreas") { "Copied!" } else { "Andreas" }}
                        </div>
                    </div>
                    <div
                        class=move || {
                            if copied_signal.get() == Some("ac") {
                                "bg-purple-200 rounded-lg p-3 text-center border-2 border-purple-400 cursor-pointer transition"
                            } else {
                                "bg-purple-50 rounded-lg p-3 text-center border border-purple-200 cursor-pointer hover:bg-purple-100 transition active:scale-95"
                            }
                        }
                        on:click=move |_| copy_to_clipboard("ac", ac_sum)
                    >
                        <div class="text-2xl font-bold text-purple-900">{format!("{:.2}", ac_sum)}</div>
                        <div class="text-xs font-semibold text-purple-700">
                            {move || if copied_signal.get() == Some("ac") { "Copied!" } else { "AC" }}
                        </div>
                    </div>
                </div>

                <div class="text-center text-sm text-gray-500 pb-2 border-b">
                    "Total: "
                    <span class="font-semibold text-gray-700">{format!("{:.2},-", total)}</span>
                </div>
            </div>

            // <!-- Linked Days -->
            {days
                .as_ref()
                .map(|days_list| {
                    view! {
                        <div class="mb-4 pb-4 border-b">
                            <h4 class="text-sm font-semibold text-gray-700 mb-2">"Linked to Days:"</h4>
                            <div class="flex flex-wrap gap-2">
                                {days_list
                                    .iter()
                                    .map(|day| {
                                        view! {
                                            <span class="px-3 py-1 bg-gray-100 text-gray-700 text-sm rounded-full border border-gray-300">
                                                {format!("{}", day.date)}
                                            </span>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                })}

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
