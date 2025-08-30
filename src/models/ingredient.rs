#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[derive(serde::Deserialize)]
#[cfg_attr(feature = "ssr", derive(Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::ingredients))]
pub struct IngredientForm<'a> {
    pub name: &'a str,
    pub amount: i32,
    pub bought: bool,
    pub meal_id: Option<i32>,
}

#[cfg_attr(feature = "ssr", derive(Queryable, Selectable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::ingredients))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub bought: bool,
    pub meal_id: Option<i32>,
}