use crate::models::days_ingredients::DayIngredient;
use leptos::prelude::*;

#[server]
pub async fn upsert_day_ingredient(
    day_ingredient: DayIngredient,
) -> Result<DayIngredient, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        insert_into(days_ingredients::table)
            .values(&day_ingredient)
            .on_conflict((days_ingredients::day_id, days_ingredients::ingredient_id))
            .do_update()
            .set(&day_ingredient)
            .get_result(db),
        "Could not update day_ingredient {day_ingredient:?}"
    )
}

#[server]
pub async fn insert_day_ingredient(
    day_ingredient: DayIngredient,
) -> Result<DayIngredient, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        insert_into(days_ingredients::table)
            .values(&day_ingredient)
            .on_conflict((days_ingredients::day_id, days_ingredients::ingredient_id))
            .do_nothing()
            .get_result(db),
        "Could not update day_ingredient {day_ingredient:?}"
    )
}

#[server]
pub async fn delete_day_ingredient_for_day(
    day_id: i32
) -> Result<usize, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        delete(days_ingredients::table)
            .filter(days_ingredients::day_id.eq(day_id))
            .execute(db),
        "Could not delete day_ingredients for {day_id}"
    )
}
