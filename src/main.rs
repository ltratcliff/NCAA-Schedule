pub mod structs;
use std::{collections::HashMap, fs};

use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};
use clap::Parser;

use crate::structs::Root;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// YYYYMMDD
    day: String,
}

fn main() {
    let args = Args::parse();
    let c = reqwest::blocking::Client::new();
    let api_url = format!(
        "https://site.api.espn.com/apis/site/v2/sports/football/college-football/scoreboard?dates={}",
        args.day
    );
    let response = c.get(api_url).send().unwrap().text().unwrap();
    let mut h: HashMap<String, DateTime<Local>> = HashMap::new();
    let mut ranks: (i64, i64) = (0, 0);
    //let f = serde_json::from_str::<Root>(&fs::read_to_string("data.json").unwrap()).unwrap();
    let f: Root = serde_json::from_str(&response).unwrap();
    for e in f.events.into_iter() {
        
        ranks.0 = e.competitions[0].competitors[0].curated_rank.current;
        ranks.1 = e.competitions[0].competitors[1].curated_rank.current;

        if ranks.0 < 26 || ranks.1 < 26 {
            let naive = NaiveDateTime::parse_from_str(&e.date, "%Y-%m-%dT%H:%MZ").unwrap();
            let d: DateTime<Local> = Local.from_utc_datetime(&naive);
            h.entry(e.name).or_insert(d);
        }

        
        //println!("{:<12} on {}", e.short_name, d.format("%I:%M %p"));
    }
    let mut hash_vec: Vec<(&String, &DateTime<Local>)> = h.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1).reverse());
    for (k, v) in hash_vec {
        println!("{:<60}  {}", k, v.format("%I:%M %p"));
    }
}
