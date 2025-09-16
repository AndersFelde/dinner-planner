#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[cfg(feature = "ssr")]
use crate::api::ssr::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::ingredients))]
pub struct IngredientForm {
    pub name: String,
    pub amount: i32,
    pub meal_id: i32,
}

#[cfg(feature = "ssr")]
impl IngredientForm {
    pub fn insert(&self, db: &mut DbConn) -> Result<Ingredient, Error> {
        insert_into(ingredients::table).values(self).get_result(db)
    }
}

#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, Identifiable, Associations, PartialEq)
)]
#[cfg_attr(feature = "ssr", diesel(belongs_to(crate::models::meal::Meal)))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::ingredients))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub meal_id: i32,
}

#[cfg(feature = "ssr")]
impl Ingredient {
    pub fn delete_for_meal(db: &mut DbConn, meal_id: i32) -> Result<usize, Error> {
        delete(ingredients::table)
            .filter(ingredients::meal_id.eq(meal_id))
            .execute(db)
    }
    pub fn get_all(db: &mut DbConn) -> Result<Vec<Ingredient>, Error> {
        ingredients::table.select(Ingredient::as_select()).load(db)
    }
    pub fn get(db: &mut DbConn, id: i32) -> Result<Ingredient, Error> {
        ingredients::table.filter(ingredients::id.eq(id)).select(Ingredient::as_select()).first(db)
    }
    pub fn get_for_meal(db: &mut DbConn, meal_id: i32) -> Result<Vec<Ingredient>, Error> {
        ingredients::table.filter(ingredients::meal_id.eq(meal_id)).select(Ingredient::as_select()).load(db)
    }
}
