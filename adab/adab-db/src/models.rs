use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub song: String,
    pub band: String,
    pub link: Option<String>,
    pub album_250px: Option<String>,
    pub album_56px: Option<String>,
    pub date: chrono::NaiveDate,
    pub source_id: i32,
}

impl Entry {
    pub fn is_duplicate(e1: &Entry, e2: &Entry) -> bool {
        e1.song == e2.song && e1.band == e2.band || e1.date == e2.date
    }
}

