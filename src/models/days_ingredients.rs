use crate::models::{
    day::Day,
    ingredient::Ingredient,
    meal::Meal,
};
#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[cfg(feature = "ssr")]
use crate::api::ssr::*;

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

#[cfg(feature = "ssr")]
impl DayIngredient {
    pub fn get_all(db: &mut DbConn) -> Result<Vec<DayIngredient>, Error>{
        days_ingredients::table.select(DayIngredient::as_select()).load(db)
    }
    pub fn update(&self, db: &mut DbConn) -> Result<DayIngredient, Error>{
        self.save_changes(db)
    }
    pub fn insert(&self, db: &mut DbConn) -> Result<DayIngredient, Error>{
        insert_into(days_ingredients::table)
            .values(self)
            .get_result(db)
    }
    pub fn delete_for_day(db: &mut DbConn, id: i32) -> Result<usize, Error>{
        delete(days_ingredients::table)
            .filter(days_ingredients::day_id.eq(id))
            .execute(db)
    }
}
