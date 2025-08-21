use chrono::NaiveDate;
use diesel::associations::HasTable;
use diesel::prelude::*;
mod models;
use maud::html;
use models::Entry;

mod schema;

#[derive(Debug, thiserror::Error)]
enum Error {
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

fn include_entry(db: &mut SqliteConnection, e: Entry) -> Result<()> {
    use self::schema::entries::dsl::*;
    diesel::insert_into(entries::table())
        .values(e)
        .execute(db)?;
    Ok(())
}

fn list_entries(db: &mut SqliteConnection) -> Result<Vec<Entry>> {
    use self::schema::entries::dsl::*;
    entries
        .select(Entry::as_select())
        .load(db)
        .map_err(Error::from)
}

fn make_page(entries: &[Entry]) -> maud::PreEscaped<String> {
    html!{
        table {
            tr {
                th { "Song" }
                th { "Band" }
                th { "Day" }
                th colspan="2" { "Links" }
            }
            @for entry in entries {
                (entry)
            }
        }
    }
}

struct Cli {
    song: String,
    band: Option<String>,
    ignore_deezer: bool,
    ignore_spotify: bool,
}

fn make_args() {
    enum Status {
        Song(String),
        Band(String, String),
    }
    let mut s = Status::Song(String::new());
    for arg in std::env::args() {

    }
}

//fn main() -> Result<()> {
//    dotenvy::dotenv()?;
//    let mut con = SqliteConnection::establish(&std::env::var("DATABASE_URL")?)?;
//
//    let entries = list_entries(&mut con)?;
//    println!("{entries:?}");
//    let page = make_page(&entries);
//    //println!("{}", page.render().into_string());
//
//    Ok(())
//}

