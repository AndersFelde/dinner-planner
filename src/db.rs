use diesel::prelude::*;
use diesel::r2d2::Error as R2D2Error;
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
use diesel::connection::SimpleConnection;

#[derive(Debug)]
struct SqliteForeignKeyEnforcer;

impl CustomizeConnection<SqliteConnection, R2D2Error> for SqliteForeignKeyEnforcer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), R2D2Error> {
        let result = (|| -> Result<(), _>{
            // Enable strict foreign key checking
            conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            // sleep if the database is busy, this corresponds to up to 2 seconds sleeping time.
            conn.batch_execute("PRAGMA busy_timeout = 2000;")?;
            // better write-concurrency
            conn.batch_execute("PRAGMA journal_mode = WAL;")?;
            // fsync only in critical moments
            conn.batch_execute("PRAGMA synchronous = NORMAL;")?;
            // write WAL changes back every 1000 pages, for an in average 1MB WAL file.
            // May affect readers if number is increased
            conn.batch_execute("PRAGMA wal_autocheckpoint = 1000;")?;
            // free some space by truncating possibly massive WAL files from the last run
            conn.batch_execute("PRAGMA wal_checkpoint(TRUNCATE);")
        })();//
        result.map_err(|e| R2D2Error::QueryError(e))
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

pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[cfg(feature = "ssr")]
#[cfg(test)]
pub mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Arc;
    pub static TEST_POOL: Lazy<Arc<Db>> = Lazy::new(|| {
        let database_url = "test-db.sqlite3"; // or "file:test-db.sqlite3?mode=memory&cache=shared"
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        let pool = Pool::builder()
            .connection_customizer(Box::new(SqliteForeignKeyEnforcer))
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");

        // Run migrations once at startup
        let mut conn = pool.get().unwrap();
        run_migrations(&mut conn).unwrap();

        Arc::new(pool)
    });

}
