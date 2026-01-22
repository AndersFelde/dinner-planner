use leptos::prelude::*;

use crate::models::receipt::ReceiptWithItems;

#[component]
pub fn Receipt<'a> (receipt_with_items: &'a ReceiptWithItems) -> impl IntoView {
    let receipt = &receipt_with_items.receipt;
    let items = &receipt_with_items.items;

    view! {
        <div class="max-w-lg mx-auto my-6 rounded-xl border border-gray-200 bg-white p-6 shadow-md">
            // <!-- Receipt Header -->
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-xl font-semibold text-gray-900">{receipt.store.clone()}</h3>
                <span class="text-gray-700 font-medium">{format!("{:.2},-", receipt.total)}</span>
            </div>

            // <!-- Date -->
            <p class="text-sm text-gray-500 mb-4">
                {format!("{}", receipt.datetime.format("%Y-%m-%d %H:%M"))}
            </p>

            // <!-- Items Table -->
            <div class="overflow-x-auto">
                <table class="w-full text-left text-sm border-collapse">
                    <thead>
                        <tr class="border-b border-gray-200">
                            <th class="py-2 px-4 text-gray-700 font-medium">"Item"</th>
                            <th class="py-2 px-4 text-gray-700 font-medium">"Price"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {items.into_iter().map(|item| view! { 
                            <tr class="border-b border-gray-100 hover:bg-gray-50">
                                <td class="py-2 px-4">{item.name.clone()}</td>
                                <td class="py-2 px-4">{format!("{:.2},-", item.price)}</td>
                            </tr>
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
