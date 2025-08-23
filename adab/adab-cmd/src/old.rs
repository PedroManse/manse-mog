fn load_old_songs(con: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    for song in SONGS {
        cannon(*song, con)?;
    }
    Ok(())
}

fn cannon(
    (day, month, song): (u32, u32, &str),
    con: &mut db::diesel::SqliteConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let track = search::search_track(song)?
        .into_iter()
        .next()
        .map(|t| Track::new(t, NaiveDate::from_ymd(2025, month, day)))
        .unwrap();
    if !db::check_can_add(con, &track.name)? {
        println!("[REPEAT] {song} not included")
    } else {
        db::include_entry(con, track.clone().into_entry())?;
        println!("[OK]: {} included", track.name);
    }
    Ok(())
}

const SONGS: &[(u32, u32, &str)] = &[
    // (day, month, song name)
];

