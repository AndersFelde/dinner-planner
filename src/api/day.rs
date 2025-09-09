use crate::api::days_ingredients::delete_day_ingredient_for_day;
use crate::api::meal::get_meal;
use crate::models::day::{Day, DayForm, DayWithMeal};
use crate::models::meal::{Meal, MealWithIngredients};
use chrono::{Datelike, Local, NaiveDate, Weekday};
use leptos::prelude::*;
use leptos::logging::log;

#[server]
pub async fn get_day(id: i32) -> Result<Day, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        days::table.filter(days::id.eq(id)).first::<Day>(db),
        "Could not get day {id}"
    )
}
#[cfg(feature = "ssr")]
pub async fn get_days_for_meal(meal_id: i32) -> Result<Vec<Day>, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        days::table.filter(days::meal_id.eq(meal_id)).load(db),
        "Could not get day meal {meal_id}"
    )
}

#[server]
pub async fn upsert_day(day_form: DayForm) -> Result<(), ServerFnError> {
    use crate::api::days_ingredients::insert_day_ingredient;
    use crate::api::ingredient::get_ingredients_for_meal;
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    let day_id = server_err!(
        insert_into(days::table)
            .values(&day_form)
            .on_conflict(days::date)
            .do_update()
            .set(&day_form)
            .get_result::<Day>(db),
        "Could not create day with {day_form:?}"
    )?
    .id;
    delete_day_ingredient_for_day(day_id).await?;
    if let Some(meal_id) = day_form.meal_id {
        for ingredient in get_ingredients_for_meal(db, meal_id)? {
            insert_day_ingredient(DayIngredient {
                day_id: day_id,
                ingredient_id: ingredient.id,
                bought: false,
            }).await?;
        }
    }
    Ok(())
}
