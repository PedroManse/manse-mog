use maud::{Markup, Render, html};

pub fn page(title: &'static str, head_items: &[Markup], content: Markup) -> Markup {
    html! {
        (maud::DOCTYPE);
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=1, initial-scale=1.0";
                (CSS("style.css"));
                @for head in head_items {
                    (head);
                }
                title { (title) }
            }
            body { (content) }
        }
    }
}

pub fn adab_table(tracks: Vec<crate::Track>) -> Markup {
    html! {
        div {
            table {
                tr {
                    th { "day" }
                    th { "track" }
                    th { "artist" }
                    th { "link" }
                }
                @for track in tracks {
                    (track);
                }
            }
            div.cats {
                @for cat in CATS {
                    img src=(cat);
                }
            }
        }
    }
}

const CATS: &[&str] = &[
    "https://media.tenor.com/3l2spchssoMAAAAm/happy-mothers-day.webp",
    "https://media.tenor.com/abdvZmOX64QAAAAi/cat-cat-meme.gif",
    "https://media.tenor.com/HAU_nZjbw9gAAAAm/cat-dance.webp",
    "https://media.tenor.com/TrZcpR0Kde8AAAAm/cat-meme-funny.webp",
    "https://media.tenor.com/KFFx4_AtN24AAAAm/pain-in-his-dih.webp",
    "https://media.tenor.com/i9lcgU4sWnkAAAAm/wawa-oh-the-misery.webp",
    "https://media.tenor.com/n52WS-CO8tsAAAAm/cat-cats.webp",
];

impl Render for crate::Track {
    fn render(&self) -> Markup {
        html! {
            tr {
                td.day { (self.day.format("%d %b")) }
                td.track { (self.name) }
                td.artist { (self.artist) }
                td.links {
                    @if let Some(link) = &self.link {
                        a href=(link) {
                            img src=(self.album_art_56px.as_ref().unwrap());
                        }
                    } @else {
                        img title="Link could not be found" src=(self.album_art_56px.as_ref().unwrap());
                    }
                }
            }
        }
    }
}

pub struct CSS(pub &'static str);
impl Render for CSS {
    fn render(&self) -> Markup {
        html! { link rel="stylesheet" type="text/css" href=(self.0) {} }
    }
}

pub struct JS(pub &'static str);
impl Render for JS {
    fn render(&self) -> Markup {
        html! { script type="application/javascript" src=(self.0) {} }
    }
}
