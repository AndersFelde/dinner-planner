pub mod day;
pub mod meal;
pub mod ingredient;
pub mod days_ingredients;
pub mod week;

#[macro_export]
macro_rules! server_err {
        ($expr:expr, $($arg:tt)*) => {
            $expr.map_err(|e| ServerFnError::new(format!("{}: {:?}", format!($($arg)*), e)))
        };
    }

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::db::*;
    pub use crate::models::{
        day::Day, days_ingredients::DayIngredient, ingredient::Ingredient, meal::Meal,
    };
    pub use crate::schema::{days, days_ingredients, ingredients, meals};
    pub use crate::server_err;
    pub use diesel::dsl::{delete, insert_into, update};
    pub use diesel::prelude::*;
    use diesel::r2d2::{ConnectionManager, PooledConnection};
    use leptos::prelude::{use_context, ServerFnError};
    pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

    pub fn get_db() -> Result<DbConn, ServerFnError> {
        server_err!(
            use_context::<Db>()
                .ok_or(ServerFnError::new("Missing Db context"))?
                .get(),
            "Failed to get DB connection"
        )
    }
}
