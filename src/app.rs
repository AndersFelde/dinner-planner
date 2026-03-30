use crate::components::notifications::Notifications;
use crate::routes::receipt::{ReceiptCreateRoute, ReceiptListRoute};
use crate::routes::{meal_list::MealList, shopping_list::ShoppingList, week::Week};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ToHref},
    path,
};
use reactive_stores::Store;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    extra_items_count: usize,
    week_ingredients_count: usize,
}

pub type IngredientUpdateMap = RwSignal<HashMap<(i32, i32), bool>>;

#[derive(Clone)]
pub enum RouteUrl {
    Home,
    MealList,
    ShoppingList,
    ReceiptCreate,
    ReceiptList,
}
impl RouteUrl {
    fn as_path(&self) -> String {
        match self {
            RouteUrl::Home => "/".to_string(),
            RouteUrl::MealList => "/meals".to_string(),
            RouteUrl::ShoppingList => "/shopping-list".to_string(),
            RouteUrl::ReceiptCreate => "/receipt".to_string(),
            RouteUrl::ReceiptList => "/receipt-list".to_string(),
        }
    }

    pub fn redirect(&self, url: String) -> String {
        format!("{}?redirect={}", self.as_path(), url)
    }
}

impl ToHref for RouteUrl {
    fn to_href(&self) -> Box<dyn Fn() -> std::string::String> {
        let path = self.as_path();
        Box::new(move || path.clone())
    }
}

impl std::fmt::Display for RouteUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path())
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <meta name="mobile-web-app-capable" content="yes" />
                <meta name="apple-mobile-web-app-status-bar-style" content="default" />
                <meta name="apple-mobile-web-app-title" content="Dinner for three" />
                // Disable auto zoom on input for ios
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, maximum-scale=1"
                />

                <link rel="apple-touch-icon" href="https://i.ibb.co/5XS4mWSy/icon.png" />
                <link rel="manifest" href="/manifest.json" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(Store::new(GlobalState::default()));

    let ingredient_updates: IngredientUpdateMap = RwSignal::new(HashMap::new());
    provide_context(ingredient_updates);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dinner-planner.css" />

        // sets the document title
        <Title text="Dinner for three" />

        // content for this welcome page
        <Notifications />
        <WsListener />
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=Week />
                    <Route path=path!("/shopping-list") view=ShoppingList />
                    <Route path=path!("/meals") view=MealList />
                    <Route path=path!("/receipt") view=ReceiptCreateRoute />
                    <Route path=path!("/receipt-list") view=ReceiptListRoute />

                </Routes>
            </main>
        </Router>
    }
}

/// Invisible component that holds a single WebSocket connection for the lifetime
/// of the app session. Receives ingredient-update messages and writes them into
/// the shared `IngredientUpdateMap` context so every `DayIngredient` component
/// can react in real time.
#[component]
fn WsListener() -> impl IntoView {
    // Derive the WebSocket URL from the current page's origin at runtime so the
    // same binary works in dev (ws://localhost:3000) and prod (wss://yourdomain).
    // The entire block is compiled out under SSR.
    #[cfg(not(feature = "ssr"))]
    {
        use crate::ws::IngredientUpdate;
        use codee::string::FromToStringCodec;
        use leptos_use::{use_websocket, UseWebSocketReturn};

        let ingredient_updates = expect_context::<IngredientUpdateMap>();

        let ws_url = {
            let location = web_sys::window().expect("no window").location();
            let protocol = if location.protocol().unwrap_or_default() == "https:" {
                "wss"
            } else {
                "ws"
            };
            let host = location.host().unwrap_or_default();
            format!("{protocol}://{host}/ws")
        };

        let UseWebSocketReturn { message, .. } =
            use_websocket::<String, String, FromToStringCodec>(&ws_url);

        Effect::new(move |_| {
            if let Some(msg) = message.get() {
                if let Ok(update) = serde_json::from_str::<IngredientUpdate>(&msg) {
                    ingredient_updates.update(|map| {
                        map.insert((update.day_id, update.ingredient_id), update.bought);
                    });
                }
            }
        });
    }

    view! {}
}
