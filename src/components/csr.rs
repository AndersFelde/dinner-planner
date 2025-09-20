// #[cfg(not(feature = "ssr"))]
#[allow(unused)]
pub enum NotificationStatus {
    Granted,
    Denied,
    Default,
    NotAvailable,
}

#[cfg(feature = "hydrate")]
pub mod js {
    use crate::components::csr::NotificationStatus;
    use js_sys::{Function, Promise, Reflect};
    use leptos::task::spawn_local;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Notification;
    use web_sys::{window, Navigator, NotificationPermission};

    // ----------- Permission Helpers -----------

    pub fn request_notification_permission() {
        // Call Notification.requestPermission()
        if has_notification_api() {
            spawn_local(async {
                match Notification::request_permission() {
                    Ok(promise) => {
                        let _ = JsFuture::from(promise).await;
                    }
                    Err(_) => {}
                }
            });
        }
    }

    pub fn has_notification_api() -> bool {
        if let Some(win) = window() {
            Reflect::has(&win, &"Notification".into()).unwrap_or(false)
        } else {
            false
        }
    }

    pub fn check_notification_permission() -> NotificationStatus {
        if has_notification_api() {
            match Notification::permission() {
                NotificationPermission::Granted => NotificationStatus::Granted,
                NotificationPermission::Default => NotificationStatus::Default,
                NotificationPermission::Denied => NotificationStatus::Denied,
                _ => NotificationStatus::Default,
            }
        } else {
            NotificationStatus::NotAvailable
        }
    }
    fn get_badge_fn(name: &str) -> Option<(Navigator, Function)> {
        let nav = window()?.navigator();
        let nav_js = JsValue::from(&nav);
        let f = Reflect::get(&nav_js, &JsValue::from_str(name))
            .ok()?
            .dyn_into::<Function>()
            .ok()?;
        Some((nav, f))
    }

    pub fn set_badge(count: usize) {
        if let Some((nav, f)) = get_badge_fn("setAppBadge") {
            let _ = f
                .call1(&nav, &JsValue::from_f64(count as f64))
                .map(|promise| {
                    if let Ok(promise) = promise.dyn_into::<Promise>() {
                        spawn_local(async {
                            let _ = JsFuture::from(promise).await;
                        });
                    }
                });
        }
    }

    #[allow(unused)]
    pub fn clear_badge() {
        if let Some((nav, f)) = get_badge_fn("clearAppBadge") {
            let _ = f.call0(&nav).map(|promise| {
                if let Ok(promise) = promise.dyn_into::<Promise>() {
                    spawn_local(async {
                        let _ = JsFuture::from(promise).await;
                    });
                }
            });
        }
    }
}

#[cfg(not(feature = "hydrate"))]
pub mod js {
    use crate::components::csr::NotificationStatus;

    pub fn set_badge(_: usize) {}

    #[allow(unused)]
    pub fn clear_badge() {}
    pub fn check_notification_permission() -> NotificationStatus {
        NotificationStatus::NotAvailable
    }
    pub fn request_notification_permission() {}
}
