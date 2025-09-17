use chrono::NaiveDate;
#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[cfg(feature = "ssr")]
use crate::api::ssr::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::days))]
#[cfg_attr(feature = "ssr", diesel(treat_none_as_null = true))]
pub struct DayForm {
    pub date: NaiveDate,
    pub meal_id: Option<i32>,
    pub week: i32,
    pub year: i32,
}

#[cfg(feature = "ssr")]
impl DayForm {
    pub fn upsert(&self, db: &mut DbConn) -> Result<Day, Error> {
        insert_into(days::table)
            .values(self)
            .on_conflict(days::date)
            .do_update()
            .set(self)
            .get_result::<Day>(db)
    }
}

#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, Identifiable, Associations)
)]
#[cfg_attr(feature = "ssr", diesel(belongs_to(crate::models::meal::Meal)))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::days))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct Day {
    pub id: i32,
    pub date: NaiveDate,
    pub meal_id: Option<i32>,
    pub week: i32,
    pub year: i32,
}

#[cfg(feature = "ssr")]
impl Day {
    pub fn get(db: &mut DbConn, id: i32) -> Result<Day, Error> {
        days::table.filter(days::id.eq(id)).first::<Day>(db)
    }
    pub fn get_all(db: &mut DbConn) -> Result<Vec<Day>, Error> {
        days::table.select(Day::as_select()).load(db)
    }

    pub fn get_for_meal(db: &mut DbConn, id: i32) -> Result<Vec<Day>, Error>{
        days::table.filter(days::meal_id.eq(id)).load(db)
    }

}
