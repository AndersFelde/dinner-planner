use crate::models::ingredient::Ingredient;

#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[cfg(feature = "ssr")]
use crate::api::ssr::*;

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

#[cfg(feature = "ssr")]
impl MealForm {
    pub fn insert(&self, db: &mut DbConn) -> Result<Meal, Error> {
        insert_into(meals::table).values(self).get_result(db)
    }
}

#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, AsChangeset, Identifiable)
)]
#[cfg_attr(feature = "ssr", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::meals))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]

pub struct Meal {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub recipie_url: Option<String>,
}

#[cfg(feature = "ssr")]
impl Meal {
    pub fn get_all(db: &mut DbConn) -> Result<Vec<Meal>, Error> {
        meals::table.select(Meal::as_select()).load(db)
    }
    pub fn get_all_ordered(db: &mut DbConn) -> Result<Vec<Meal>, Error> {
        use diesel::dsl::sql;
        meals::table
            .select(Meal::as_select())
            // Hacky but to make it case insensitive
            .order(sql::<diesel::sql_types::Text>("name COLLATE NOCASE ASC"))
            // .order(meals::name.desc())
            .load(db)
    }
    pub fn get(db: &mut DbConn, id: i32) -> Result<Meal, Error> {
        meals::table.filter(meals::id.eq(id)).first::<Meal>(db)
    }

    pub fn delete(db: &mut DbConn, id: i32) -> Result<usize, Error> {
        delete(meals::table).filter(meals::id.eq(id)).execute(db)
    }

    pub fn update(&self, db: &mut DbConn) -> Result<Meal, Error> {
        self.save_changes(db)
    }
}
