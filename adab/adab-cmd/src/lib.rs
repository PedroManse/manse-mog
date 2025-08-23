pub use adab_db as db;
pub use adab_search as search;
pub mod html;

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub artist: String,
    pub link: Option<String>,
    pub album_art_250px: Option<String>,
    pub album_art_56px: Option<String>,
    pub day: chrono::NaiveDate,
}

impl Track {
    pub fn from_today(track: search::Track) -> Self {
        Self::new(track, chrono::Utc::now().date_naive())
    }
    pub fn new(t: search::Track, day: chrono::NaiveDate) -> Self {
        Self {
            name: t.title,
            artist: t.artist.name,
            link: t.link,
            album_art_56px: Some(t.album.cover_small),
            album_art_250px: Some(t.album.cover_medium),
            day,
        }
    }
    pub fn into_entry(self) -> db::models::Entry {
        db::models::Entry {
            song: self.name,
            band: self.artist,
            link: self.link,
            album_250px: self.album_art_250px,
            album_56px: self.album_art_56px,
            date: self.day,
            source_id: 1,
        }
    }
}
