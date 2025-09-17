use leptos::prelude::*;
#[component]
pub fn Modal(show: ReadSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <Show when=move || show.get() fallback=|| view! {}>
            <div
                id="default-modal"
                tabindex="-1"
                class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"
            >
                <div class="relative p-4 w-full max-w-2xl max-h-full">{children.read_value()()}</div>
            </div>
        </Show>
    }
}
