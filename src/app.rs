use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ToHref}, path, AsPath, ParamSegment, PossibleRouteMatch, StaticSegment
};
use crate::components::{week::*, meal_form::*, day_form::*};

// <Route path=path!("/") view=Week />
// <Route path=path!("/new/meal") view=MealForm />
// <Route path=path!("/edit/day/:id") view=DayForm />
#[derive(Clone)]
pub enum RouteUrl {
    Home,
    NewMeal,
    EditDay{id: i32},
}
impl RouteUrl {
    fn as_path(&self ) -> String {
        match self {
            RouteUrl::Home => "/".to_string(),
            RouteUrl::NewMeal => "/new/meal".to_string(),
            RouteUrl::EditDay{id} => format!("/edit/day/{id}"),
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

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dinner-planner.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=Week />
                    <Route path=path!("/new/meal") view=MealForm />
                    <Route path=path!("/edit/day/:id") view=DayForm />
                </Routes>
            </main>
        </Router>
    }
}
