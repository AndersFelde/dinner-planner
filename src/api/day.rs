use crate::models::{
        day::{Day, DayForm},
        days_ingredients::DayWithMealAndIngredients,
    };
use leptos::prelude::*;

#[server]
pub async fn get_day(id: i32) -> Result<Day, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(Day::get(db, id), "Could not get day {id}")
}
#[cfg(feature = "ssr")]
pub async fn get_days_for_meal(meal_id: i32) -> Result<Vec<Day>, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        Day::get_for_meal(db, meal_id),
        "Could not get day meal {meal_id}"
    )
}

#[server]
pub async fn upsert_day(day_form: DayForm) -> Result<DayWithMealAndIngredients, ServerFnError> {
    use crate::api::days_ingredients::{delete_day_ingredient_for_day, insert_day_ingredient};
    use crate::api::ingredient::get_ingredients_for_meal;
    use crate::models::days_ingredients::IngredientWithBought;
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    let day = server_err!(
        day_form.upsert(db),
        "Could not create day with {day_form:?}"
    )?;
    delete_day_ingredient_for_day(day.id).await?;
    let mut meal = None;
    if let Some(meal_id) = day_form.meal_id {
        let mut ingredients: Vec<IngredientWithBought> = Vec::new();
        for ingredient in get_ingredients_for_meal(db, meal_id)? {
            let ingredient = insert_day_ingredient(DayIngredient {
                day_id: day.id,
                ingredient_id: ingredient.id,
                bought: false,
            })
            .await?;
            ingredients.push(IngredientWithBought {
                day_id: day.id,
                ingredient: server_err!(
                    Ingredient::get(db, ingredient.ingredient_id),
                    "Could not get ingredient {}",
                    ingredient.ingredient_id
                )?,
                bought: ingredient.bought,
            });
        }
        meal = Some((
            server_err!(Meal::get(db, meal_id), "Could not get meal {meal_id}")?,
            ingredients,
        ));
    }
    Ok(DayWithMealAndIngredients {
        day: day,
        meal: meal,
    })
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::db::tests::TEST_POOL;
    use crate::{api::week::Week, models::meal::MealForm};
    use chrono::{Datelike, Local};
    use diesel::Connection;

    #[test]
    pub fn test_foreign_key_restraint() {
        let day = Local::now();
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
        assert!(matches!(DayForm {
                date: day.date_naive(),
                meal_id: Some(1),
                week: day.iso_week().week() as i32,
                year: day.year(),
            }.upsert(db), Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::ForeignKeyViolation, ref info)) if info.message() == "FOREIGN KEY constraint failed" ));
        Ok(())
    });
    }
    #[test]
    fn test_get_days_for_meal() {
        let day = Local::now();
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
            let meal = MealForm {
                image: String::new(),
                name: String::new(),
                recipie_url: None,
            }
            .insert(db)
            .unwrap();
            let day = DayForm {
                date: day.date_naive(),
                meal_id: Some(meal.id),
                week: day.iso_week().week() as i32,
                year: day.year(),
            }
            .upsert(db)
            .unwrap();
            assert_eq!(vec![day.clone()], Day::get_for_meal(db, meal.id).unwrap());
            assert_eq!(0, Day::get_for_meal(db, 99).unwrap().len());
            Ok(())
        });
    }
    #[test]
    fn test_get_day() {
        let day = Local::now();
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
            let day = DayForm {
                date: day.date_naive(),
                meal_id: None,
                week: day.iso_week().week() as i32,
                year: day.year(),
            }
            .upsert(db)
            .unwrap();
            assert_eq!(day, Day::get(db, day.id).unwrap());
            Ok(())
        });
    }
    #[test]
    fn test_usert_day() {
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
            let week = Week::current();
            let dates = week.dates();
            for day in dates {
                DayForm {
                    date: day,
                    meal_id: None,
                    week: day.iso_week().week() as i32,
                    year: day.year(),
                }
                .upsert(db)
                .unwrap();
            }
            let days = Day::get_all(db).unwrap();
            // Check that they  are created
            assert_eq!(days.len(), 7);
            for (i, day) in days.iter().enumerate() {
                assert_eq!(day.date, dates[i]);
                assert_eq!(day.week, (dates[i].iso_week().week()) as i32);
                assert_eq!(day.year, dates[i].year());
                assert_eq!(day.meal_id, None);
            }

            for (i, day) in dates.iter().enumerate() {
                DayForm {
                    date: day.clone(),
                    meal_id: None,
                    week: day.iso_week().week() as i32 + i as i32,
                    year: day.year() + i as i32,
                }
                .upsert(db)
                .unwrap();
            }
            let days = Day::get_all(db).unwrap();
            // Check that they are updated
            assert_eq!(days.len(), 7);
            for (i, day) in days.iter().enumerate() {
                assert_eq!(day.date, dates[i]);
                assert_eq!(day.meal_id, None);
                assert_eq!(day.week, (dates[i].iso_week().week()) as i32 + i as i32);
                assert_eq!(day.year, dates[i].year() + i as i32);
            }
            Ok(())
        });
    }
}
