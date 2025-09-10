
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::Error as R2D2Error;

#[derive(Debug)]
struct SqliteForeignKeyEnforcer;

impl CustomizeConnection<SqliteConnection, R2D2Error> for SqliteForeignKeyEnforcer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), R2D2Error> {
        diesel::sql_query("PRAGMA foreign_keys = ON;")
            .execute(conn)
            .map(|_| ())
            .map_err(|e| R2D2Error::QueryError(e))
    }
}

pub type Db = Pool<ConnectionManager<SqliteConnection>>;
pub fn get_db_pool() -> Db {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .connection_customizer(Box::new(SqliteForeignKeyEnforcer))
        .test_on_check_out(true)
        .build(manager)
        .expect("Cloud not build connection pool")
}
