use chrono::NaiveDate;
pub mod deezer;
pub mod html;

pub struct Track {
    name: String,
    artist: String,
    deezer_link: Option<String>,
    album_art_250px: String,
    album_art_56px: String,
    preview_link: String,
    day: chrono::NaiveDate,
}

impl Track {
    pub fn from_today(track: deezer::Track) -> Self {
        Self::new(track, chrono::Utc::now().date_naive())
    }
    pub fn new(t: deezer::Track, day: NaiveDate) -> Self {
        Self {
            name: t.title,
            artist: t.artist.name,
            deezer_link: t.link,
            album_art_56px: t.album.cover_small,
            album_art_250px: t.album.cover_medium,
            preview_link: t.preview,
            day,
        }
    }
}
