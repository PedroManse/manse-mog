use diesel::prelude::*;
use maud::{Render, html};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub song: String,
    pub band: String,
    pub date: chrono::NaiveDate,
    pub deezer_link: Option<String>,
    pub spotify_link: Option<String>,
}

impl Entry {
    pub fn is_duplicate(e1: &Entry, e2: &Entry) -> bool {
        e1.song == e2.song && e1.band == e2.band || e1.date == e2.date
    }
}

impl Render for Entry {
    fn render(&self) -> maud::Markup {
        html! {
            tr {
                th {
                    (self.song)
                }
                td {
                    (self.band)
                }
                td {
                    (self.date)
                }
                td {
                    @if let Some(link) = &self.deezer_link {
                        a class="deezer" href=(link){}
                    }
                }
                td {
                    @if let Some(link) = &self.spotify_link {
                        a class="spotify" href=(link){}
                    }
                }
            }
        }
    }
}
