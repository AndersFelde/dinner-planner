#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[cfg(feature = "ssr")]
use crate::api::ssr::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::extra_items))]
pub struct ExtraItemForm {
    pub name: String,
    pub amount: i32,
    pub bought: bool,
}

#[cfg(feature = "ssr")]
impl ExtraItemForm {
    pub fn insert(&self, db: &mut DbConn) -> Result<ExtraItem, Error> {
        insert_into(extra_items::table).values(self).get_result(db)
    }
}

#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, Identifiable, PartialEq, AsChangeset)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::extra_items))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ExtraItem {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub bought: bool,
}

#[cfg(feature = "ssr")]
impl ExtraItem {
    pub fn delete(db: &mut DbConn, id: i32) -> Result<usize, Error> {
        delete(extra_items::table)
            .filter(extra_items::id.eq(id))
            .execute(db)
    }
    pub fn get_all(db: &mut DbConn) -> Result<Vec<ExtraItem>, Error> {
        extra_items::table.select(ExtraItem::as_select()).load(db)
    }
    pub fn get_all_not_bought(db: &mut DbConn) -> Result<Vec<ExtraItem>, Error> {
        extra_items::table.filter(extra_items::bought.eq(false)).select(ExtraItem::as_select()).load(db)
    }
    pub fn get(db: &mut DbConn, id:i32) -> Result<ExtraItem, Error> {
        extra_items::table.filter(extra_items::id.eq(id)).first::<ExtraItem>(db)
    }
    pub fn update(&self, db: &mut DbConn) -> Result<ExtraItem, Error> {
        self.save_changes(db)
    }
}
