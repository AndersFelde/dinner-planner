pub mod models;

#[cfg(feature = "ssr")]
pub mod schema;

#[cfg(feature = "ssr")]
pub mod db;

#[cfg(feature = "ssr")]
pub mod api;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use diesel::prelude::*;
    use dinner_planner::{api::ssr::get_db, app::*};
    use leptos::logging::{error, log};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let pool = db::get_db_pool();
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    match pool.get() {
        Ok(mut con) => match diesel::sql_query("PRAGMA foreign_keys = ON").execute(&mut con) {
            Ok(_) => (),
            Err(e) => {
                error!("Could not enable foreign keys: {e}");
                return;
            }
        },
        Err(e) => {
            error!("Could not get DB pool: {e}");
            return;
        }
    }

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(pool.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
