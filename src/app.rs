use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, path, AsPath, StaticSegment
};
use crate::components::{week::*, meal_form::*, day_form::*};

#[derive(Clone)]
pub enum RouteUrl {
    Week,
    MealForm,
    DayForm,
}
impl AsPath for RouteUrl {
    fn as_path(&self ) -> &'static str{
        match self {
            RouteUrl::Week => "/",
            RouteUrl::MealForm => "/new_meal",
            RouteUrl::DayForm => "/new_day",
        }

    }
}

impl IntoAttributeValue for RouteUrl {
    type Output = &'static str;

    fn into_attribute_value(self) -> Self::Output {
        self.as_path()
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
                    <Route path=StaticSegment(RouteUrl::Week) view=Week />
                    <Route path=StaticSegment(RouteUrl::MealForm) view=MealForm />
                    <Route path=StaticSegment(RouteUrl::DayForm) view=DayForm />
                </Routes>
            </main>
        </Router>
    }
}
