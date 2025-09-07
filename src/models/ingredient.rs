#[cfg(feature = "ssr")]
use diesel::prelude::*;
use crate::models::meal::Meal;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::ingredients))]
pub struct IngredientForm {
    pub name: String,
    pub amount: i32,
    pub meal_id: Option<i32>,
}

#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Identifiable, Associations, PartialEq))]
#[cfg_attr(feature = "ssr", diesel(belongs_to(Meal)))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::ingredients))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub meal_id: i32,
}