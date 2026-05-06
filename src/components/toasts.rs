use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static NEXT_TOAST_ID: AtomicUsize = AtomicUsize::new(1);
const TOAST_SHOW_MS: u64 = 3500;
const TOAST_FADE_MS: u64 = 400;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Toast {
    pub id: usize,
    pub message: String,
}

#[derive(Clone)]
pub struct ToastStore {
    toasts: RwSignal<Vec<Toast>>,
}

impl ToastStore {
    pub fn new() -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
        }
    }

    pub fn push_error(&self, message: impl Into<String>) {
        let id = NEXT_TOAST_ID.fetch_add(1, Ordering::Relaxed);
        let message = message.into();
        self.toasts.update(|toasts| {
            toasts.push(Toast { id, message });
        });

        let toasts = self.toasts;
        set_timeout(
            move || {
                toasts.update(|toasts| {
                    toasts.retain(|toast| toast.id != id);
                });
            },
            Duration::from_millis(TOAST_SHOW_MS + TOAST_FADE_MS),
        );
    }

    pub fn list(&self) -> ReadSignal<Vec<Toast>> {
        self.toasts.read_only()
    }
}

#[component]
pub fn ToastViewport() -> impl IntoView {
    let store = expect_context::<ToastStore>();
    let toasts = store.list();
    let animation_style = format!(
        "animation: toast-fade-out {}ms ease-in forwards; animation-delay: {}ms;",
        TOAST_FADE_MS, TOAST_SHOW_MS
    );

    view! {
        <div class="fixed top-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
            <For each=move || toasts.get() key=|toast| toast.id let:toast>
                <div
                    class="pointer-events-auto bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded shadow-lg max-w-sm w-80"
                    style=animation_style.clone()
                >
                    <div class="flex items-start gap-2">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="size-5 text-red-500 mt-0.5"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"
                            />
                        </svg>
                        <div class="text-sm break-words">{toast.message.clone()}</div>
                    </div>
                </div>
            </For>
        </div>
    }
}
