use crate::models::ingredient::{Ingredient, IngredientForm};
#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MealWithIngredients {
    #[serde(flatten)]
    pub meal: Meal,
    pub ingredients: Vec<Ingredient>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::meals))]
pub struct MealForm {
    pub name: String,
    pub image: String,
    pub recipie_url: Option<String>,
}

#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, AsChangeset, Identifiable)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::meals))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]

pub struct Meal {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub recipie_url: Option<String>,
}
