use diesel::associations::HasTable;
use diesel::prelude::*;
pub mod models;
use models::Entry;
mod schema;
pub use diesel;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SQLCon(#[from] diesel::ConnectionError),
    #[error(transparent)]
    SQL(#[from] diesel::result::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    EnvFile(#[from] dotenvy::Error),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),
    #[error("Song {0} was already included")]
    DuplicateMusic(String),
}

type Result<T> = std::result::Result<T, Error>;

pub fn include_entry(db: &mut SqliteConnection, e: Entry) -> Result<()> {
    use self::schema::entries::dsl::*;
    diesel::insert_into(entries::table())
        .values(e)
        .execute(db)?;
    Ok(())
}

pub fn list_entries(db: &mut SqliteConnection) -> Result<Vec<Entry>> {
    use self::schema::entries::dsl::*;
    entries
        .select(Entry::as_select())
        .load(db)
        .map_err(Error::from)
}

pub fn check_can_add(db: &mut SqliteConnection, name: &str) -> Result<bool> {
    use self::schema::entries::dsl::*;
    let x: Vec<Entry> = entries::table()
        .select(Entry::as_select())
        .filter(song.eq(name))
        .load(db)?;
    Ok(x.is_empty())
}

pub fn con() -> Result<SqliteConnection> {
    dotenvy::dotenv()?;
    SqliteConnection::establish(&std::env::var("DATABASE_URL")?).map_err(Error::from)
}
