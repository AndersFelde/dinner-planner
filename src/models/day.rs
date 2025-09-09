use chrono::NaiveDate;
#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::days))]
pub struct DayForm {
    pub date: NaiveDate,
    pub meal_id: Option<i32>,
    pub week: i32,
    pub year: i32,
}

#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, Identifiable, Associations)
)]
#[cfg_attr(feature = "ssr", diesel(belongs_to(crate::models::meal::Meal)))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::days))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Day {
    pub id: i32,
    pub date: NaiveDate,
    pub meal_id: Option<i32>,
    pub week: i32,
    pub year: i32,
}