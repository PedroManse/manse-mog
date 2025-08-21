pub fn search_track(query: &str) -> Pagination<Track> {
    reqwest::blocking::get(format!("https://api.deezer.com/search?q={query}"))
        .unwrap()
        .json()
        .unwrap()
}

#[derive(serde::Deserialize, Debug)]
pub struct Pagination<T> {
    pub data: Vec<T>,
    pub total: u64,
}

impl<T> IntoIterator for Pagination<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'s, T> IntoIterator for &'s Pagination<T> {
    type Item = &'s T;
    type IntoIter = <std::slice::Iter<'s, T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.as_slice().into_iter()
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Track {
    pub id: u64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: String,
    pub link: Option<String>,
    pub share: Option<String>,
    pub duration: u64,
    pub release_date: Option<String>,
    pub contributors: Option<Vec<Contributor>>,
    pub artist: Artist,
    pub album: Album,
    pub entity_type: Option<EntityType>,
    pub preview: String,
}

#[derive(serde::Deserialize, Debug)]
pub enum EntityType {
    Track,
    Album,
    Artist,
}

#[derive(serde::Deserialize, Debug)]
pub struct Contributor {
    pub id: u64,
    pub name: String,
    pub link: Option<String>,
    pub share: Option<String>,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub radio: Option<bool>,
    pub tracklist: String,
    pub entity_type: Option<EntityType>,
    pub role: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub link: Option<String>,
    pub share: Option<String>,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub radio: Option<bool>,
    pub tracklist: String,
    pub entity_type: Option<EntityType>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Album {
    pub id: u64,
    pub title: String,
    pub link: Option<String>,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub md5_image: String,
    pub release_date: Option<String>,
    pub tracklist: String,
    pub entity_type: Option<EntityType>,
}
