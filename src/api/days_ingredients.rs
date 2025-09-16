use crate::models::days_ingredients::DayIngredient;
use leptos::prelude::*;

#[server]
pub async fn udpate_day_ingredient(
    day_ingredient: DayIngredient,
) -> Result<DayIngredient, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        day_ingredient.update(db),
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
        day_ingredient.insert(db),
        "Could not update day_ingredient {day_ingredient:?}"
    )
}

#[server]
pub async fn delete_day_ingredient_for_day(day_id: i32) -> Result<usize, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        DayIngredient::delete_for_day(db, day_id),
        "Could not delete day_ingredients for {day_id}"
    )
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::db::tests::TEST_POOL;
    use crate::models::day::DayForm;
    use crate::models::ingredient::IngredientForm;
    use crate::models::meal::MealForm;
    use chrono::{Datelike, Local};
    use diesel::Connection;

    #[test]
    pub fn test_foreign_key_restraint() {
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
        assert!(matches!(DayIngredient {
                day_id: 1,
                ingredient_id: 1,
                bought: false,
            }.insert(db), Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::ForeignKeyViolation, ref info)) if info.message() == "FOREIGN KEY constraint failed" ));
        Ok(())
        });
    }
    #[test]
    pub fn test_day_ingredient_insert_update_delete() {
        let day = Local::now();
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
            let day_id = DayForm {
                date: day.date_naive(),
                meal_id: None,
                week: day.iso_week().week() as i32,
                year: day.year(),
            }
            .upsert(db)
            .unwrap()
            .id;
            let meal_id = MealForm {
                name: String::new(),
                image: String::new(),
                recipie_url: None,
            }
            .insert(db)
            .unwrap()
            .id;
            let ingredient_id = IngredientForm {
                amount: 0,
                name: String::new(),
                meal_id: meal_id,
            }
            .insert(db)
            .unwrap()
            .id;
            let mut di = DayIngredient {
                day_id: day_id,
                ingredient_id: ingredient_id,
                bought: false,
            }
            .insert(db)
            .unwrap();
            let ingredient_id = IngredientForm {
                amount: 0,
                name: String::new(),
                meal_id: meal_id,
            }
            .insert(db)
            .unwrap()
            .id;
            let other_di = DayIngredient {
                day_id: day_id,
                ingredient_id: ingredient_id,
                bought: false,
            }
            .insert(db)
            .unwrap();

            di.bought = true;
            let di = di.update(db).unwrap();

            assert_eq!(di.bought, true);
            assert_eq!(other_di.bought, false);

            assert_eq!(DayIngredient::get_all(db).unwrap().len(), 2);

            DayIngredient::delete_for_day(db, day_id).unwrap();

            assert_eq!(DayIngredient::get_all(db).unwrap().len(), 0);

            Ok(())
        });
    }
}
