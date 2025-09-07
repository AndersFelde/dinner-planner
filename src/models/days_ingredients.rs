use crate::models::{
    day::{Day, DayWithMeal},
    ingredient::Ingredient,
    meal::Meal,
};
#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DayWithMealAndIngredients {
    pub day: Day,
    pub meal: Option<(Meal, Vec<IngredientWithBought>)>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct IngredientWithBought {
    pub day_id: i32,
    pub ingredient: Ingredient,
    pub bought: bool,
}

#[cfg_attr(
    feature = "ssr",
    derive(Identifiable, Insertable, Queryable, Selectable, Associations, AsChangeset)
)]
#[cfg_attr(feature = "ssr", diesel(belongs_to(Day)))]
#[cfg_attr(feature = "ssr", diesel(belongs_to(Ingredient)))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::days_ingredients))]
#[cfg_attr(feature = "ssr", diesel(primary_key(day_id, ingredient_id)))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DayIngredient {
    pub day_id: i32,
    pub ingredient_id: i32,
    pub bought: bool,
}
