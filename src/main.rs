extern crate reqwest;
extern crate select;
extern crate term;

mod page;
mod story;

use std::io::prelude::*;
use page::Page;
use term::stdout;

fn main() {
    let mut res = reqwest::get("https://lobste.rs").unwrap();
    let body = res.text().unwrap();

    let mut t = stdout().expect("Unable to connect to STDOUT");

    let page = Page::parse_str(&body);

    for story in page.stories {
        t.fg(term::color::WHITE).unwrap();
        write!(t, "{}", story.title).unwrap();

        match story.domain {
            Some(domain) => {
                t.fg(term::color::BRIGHT_YELLOW).unwrap();
                writeln!(t, " ({})", domain).unwrap();
            },
            None => writeln!(t, "").unwrap()
        }

        let score_label = match story.score {
            1 => "point",
            _ => "points"
        };

        t.fg(term::color::BRIGHT_BLACK).unwrap();
        writeln!(t, "  {} {} | via {} {}", story.score, score_label, story.author, story.timestamp).unwrap();
        
        println!("");
    }
}
