use leptos::logging::log;
use leptos::{html::Div, prelude::*};
use leptos_use::on_click_outside;
#[component]
pub fn Modal(show: Signal<bool>, children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    let close = RwSignal::new(true);
    let children_div = NodeRef::<Div>::new();
    Effect::watch(
        move || show.get(),
        move |show, _, _| {
                close.set(!*show);
        },
        false,
    );

    let _ = on_click_outside(children_div, move |_| close.set(true));
    view! {
        <Show
            when=move || {
                !close.get()
            }
            fallback=|| view! {}
        >

            <div
                tabindex="-1"
                class="fixed inset-0 z-99 flex justify-center items-start w-full h-full pt-16 overflow-y-auto"
            >
                // on:click=move |_| show.set(false)
                <div class="absolute inset-0 bg-gray-900/60"></div>
                <div
                    node_ref=children_div
                    class="relative z-10 flex justify-center items-start w-full"
                >
                    // on:click=|e| e.stop_propagation()
                    {children.read_value()()}
                </div>
            </div>
        </Show>
    }
}
