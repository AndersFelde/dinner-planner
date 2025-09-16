#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
use crate::models::ingredient::{Ingredient, IngredientForm};

#[cfg(feature = "ssr")]
pub fn insert_ingredient(
    db: &mut crate::api::ssr::DbConn,
    ingredient_form: IngredientForm,
) -> Result<Ingredient, ServerFnError> {
    use crate::api::ssr::*;
    server_err!(
        ingredient_form.insert(db),
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
        Ingredient::delete_for_meal(db, meal_id),
        "Could not delete ingredients for meal_id {meal_id}",
    )
}

#[cfg(feature = "ssr")]
pub fn get_ingredients(db: &mut crate::api::ssr::DbConn) -> Result<Vec<Ingredient>, ServerFnError> {
    use crate::api::ssr::*;

    server_err!(Ingredient::get_all(db), "Could not get ingredients")
}

#[cfg(feature = "ssr")]
pub fn get_ingredients_for_meal(
    db: &mut crate::api::ssr::DbConn,
    meal_id: i32,
) -> Result<Vec<Ingredient>, ServerFnError> {
    use crate::api::ssr::*;

    server_err!(
        Ingredient::get_for_meal(db, meal_id),
        "Could not get ingredients for meal {meal_id}"
    )
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::db::tests::TEST_POOL;
    use crate::models::meal::MealForm;
    use diesel::Connection;

    #[test]
    pub fn test_foreign_key_restraint() {
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
        assert!(matches!(IngredientForm {
                amount: 0,
                meal_id: 0,
                name: String::new(),
            }.insert(db), Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::ForeignKeyViolation, ref info)) if info.message() == "FOREIGN KEY constraint failed" ));
        Ok(())
        });
    }
    #[test]
    pub fn test_extra_items_all() {
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
            let meal_id = MealForm {
                name: String::new(),
                image: String::new(),
                recipie_url: None,
            }
            .insert(db)
            .unwrap()
            .id;
            let meal_id_other = MealForm {
                name: String::new(),
                image: String::new(),
                recipie_url: None,
            }
            .insert(db)
            .unwrap()
            .id;

            let ingredient = IngredientForm {
                amount: 0,
                meal_id,
                name: String::new(),
            }
            .insert(db)
            .unwrap();

            let ingredient_other = IngredientForm {
                amount: 1,
                meal_id: meal_id_other,
                name: String::new(),
            }
            .insert(db)
            .unwrap();

            assert_eq!(Ingredient::get_all(db).unwrap().len(), 2);
            assert_eq!(ingredient, Ingredient::get(db, ingredient.id).unwrap());
            assert_eq!(vec![ingredient.clone()], Ingredient::get_for_meal(db, meal_id).unwrap());
            assert_ne!(
                Ingredient::get(db, ingredient.id).unwrap().amount,
                Ingredient::get(db, ingredient_other.id).unwrap().amount
            );
            Ingredient::delete_for_meal(db, meal_id_other).unwrap();
            assert_eq!(Ingredient::get_all(db).unwrap().len(), 1);
            assert_eq!(Ingredient::get_all(db).unwrap(), vec![ingredient]);

            Ok(())
        });
    }
}
