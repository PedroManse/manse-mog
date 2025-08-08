use adab_search::*;

fn main() {

    let tracks: Vec<_> = deezer::search_track("Holding out for a hero")
        .into_iter()
        .map(Track::from_today)
        .collect();
    let page = adab_search::html::page(
        "Another day, another banger",
        &[],
        adab_search::html::adab_table(tracks)
    );
    println!("{}", page.into_string());
}
