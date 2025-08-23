use std::io::BufRead;

use adab_cmd::*;
use chrono::{Datelike, NaiveDate, Utc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1).peekable();
    let mut song;
    let mut date = Utc::now().date_naive();
    match args.next() {
        Some(a) if a.starts_with("-d") => {
            let (day, month) = a
                .strip_prefix("-d")
                .unwrap()
                .split_once('.')
                .expect("date arg must be -d<day>.<month>");
            date = NaiveDate::from_ymd_opt(date.year(), month.parse()?, day.parse()?).unwrap();
            song = String::new();
        }
        Some(a) => {
            song = a;
            song.push(' ');
        }
        None => {
            panic!()
        }
    };
    for a in args {
        song.push_str(&a);
        song.push(' ');
    }

    let tracks = search::search_track(&song, 3)?;
    for (n, track) in (&tracks).into_iter().take(3).enumerate() {
        println!("#{}: {} - {}", n+1, track.title, track.artist.name);
    }
    let stdin = std::io::stdin();
    print!(">");
    let line = stdin.lock().lines().next().unwrap()?;
    let select = match line.as_str() {
        "" | "y" | "Y" | "1" => 0,
        "2" => 1,
        "3" => 2,
        "N" | "n" => return Ok(()),
        s => panic!("expects 1, 2, 3, y or n; not {s}"),
    };
    let track = Track::new(tracks.into_iter().nth(select).unwrap(), date);
    let entry = track.clone().into_entry();
    let mut con = db::con()?;
    if db::check_can_add(&mut con, &entry.song)? {
        db::include_entry(&mut con, entry)?;
        println!("[OK]: Song {} included", track.name);
    } else {
        println!("[ERR]: Song {} already included", track.name);
    }
    Ok(())
}
