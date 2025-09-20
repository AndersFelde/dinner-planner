use crate::components::{
    meal_list::MealList, notifications::Notifications,
    shopping_list::ShoppingList, week::*,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ToHref},
    path,
};

// <Route path=path!("/") view=Week />
// <Route path=path!("/new/meal") view=MealForm />
// <Route path=path!("/edit/day/:id") view=DayForm />
use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    extra_items_count: usize,
    week_ingredients_count: usize
}

#[derive(Clone)]
pub enum RouteUrl {
    Home,
    // NewMeal,
    // NewExtraItem,
    MealList,
    // EditDay { id: i32 },
    // EditExtraItem { id: i32 },
    // EditMeal { id: i32 },
    ShoppingList,
}
impl RouteUrl {
    fn as_path(&self) -> String {
        match self {
            RouteUrl::Home => "/".to_string(),
            // RouteUrl::NewMeal => "/new/meal".to_string(),
            RouteUrl::MealList => "/meals".to_string(),
            // RouteUrl::EditDay { id } => format!("/edit/day/{id}"),
            // RouteUrl::EditMeal { id } => format!("/edit/meal/{id}"),
            // RouteUrl::EditExtraItem { id } => format!("/edit/extra-item/{id}"),
            // RouteUrl::NewExtraItem => "/new/extra-item".to_string(),
            RouteUrl::ShoppingList =>"/shopping-list".to_string()
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

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dinner-planner.css" />

        // sets the document title
        <Title text="Dinner for three" />

        // content for this welcome page
        <Notifications />
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=Week />
                    // <Route path=path!("/new/meal") view=CreateMealForm />
                    // <Route path=path!("/edit/meal/:id") view=UpdateMealForm />
                    // <Route path=path!("/new/extra-item") view=CreateExtraItemForm />
                    // <Route path=path!("/edit/extra-item/:id") view=UpdateExtraItemForm />
                    // <Route path=path!("/edit/day/:id") view=DayForm />
                    <Route path=path!("/shopping-list") view=ShoppingList />
                    <Route path=path!("/meals") view=MealList />

                </Routes>
            </main>
        </Router>
    }
}
