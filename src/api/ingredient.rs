#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
use crate::models::ingredient::{IngredientForm, Ingredient};

#[cfg(feature = "ssr")]
pub fn insert_ingredient(
    db: &mut crate::api::ssr::DbConn,
    ingredient_form: IngredientForm,
) -> Result<Ingredient, ServerFnError> {
    use crate::api::ssr::*;
    server_err!(
        insert_into(ingredients::table)
            .values(&ingredient_form)
            .get_result(db),
        "Could not insert ingredient {:?}",
        ingredient_form
    )
}

#[cfg(feature = "ssr")]
pub fn delete_ingredients(
    db: &mut crate::api::ssr::DbConn,
    meal_id: i32,
) -> Result<usize, ServerFnError> {
    use crate::api::ssr::*;

    server_err!(
        delete(ingredients::table)
            .filter(ingredients::meal_id.eq(meal_id))
            .execute(db),
        "Could not delete ingredients for meal_id {meal_id}",
    )
}

#[cfg(feature = "ssr")]
pub fn get_ingredients(
    db: &mut crate::api::ssr::DbConn,
) -> Result<Vec<Ingredient>, ServerFnError> {
    use crate::api::ssr::*;

    server_err!(
        ingredients::table.select(Ingredient::as_select()).load(db),
        "Could not get ingredients"
    )
}

#[cfg(feature = "ssr")]
pub fn get_ingredients_for_meal(
    db: &mut crate::api::ssr::DbConn,
    meal_id: i32
) -> Result<Vec<Ingredient>, ServerFnError> {
    use crate::api::ssr::*;

    server_err!(
        ingredients::table.filter(ingredients::meal_id.eq(meal_id)).select(Ingredient::as_select()).load(db),
        "Could not get ingredients for meal {meal_id}"
    )
}
