pub mod structs;

use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};
use clap::Parser;
use colored::{ColoredString, Colorize};

use crate::structs::Root;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// YYYYMMDD
    day: String,
}

#[derive(Debug)]
struct Event {
    home_team: ColoredString,
    away_team: ColoredString,
    home_score: ColoredString,
    away_score: ColoredString,
    home_rank: i64,
    away_rank: i64,
    date: DateTime<Local>,
}

fn main() {
    let args = Args::parse();
    let c = reqwest::blocking::Client::new();
    let api_url = format!(
        "https://site.api.espn.com/apis/site/v2/sports/football/college-football/scoreboard?dates={}",
        args.day
    );
    let response = c.get(api_url).send().unwrap().text().unwrap();
    let deserial: Root = serde_json::from_str(&response).unwrap();
    let events = deserial.events;
    let mut sorted_games: Vec<Event> = Vec::new();
    for e in events {
        let team1_rank = e.competitions[0].competitors[0].curated_rank.current;
        let team2_rank = e.competitions[0].competitors[1].curated_rank.current;
        if !(team1_rank < 26 || team2_rank < 26 || e.name.contains("Florida State")) {
            continue;
        }
        let naive = NaiveDateTime::parse_from_str(&e.date, "%Y-%m-%dT%H:%MZ").unwrap();
        let d: DateTime<Local> = Local.from_utc_datetime(&naive);
        let mut home_team: ColoredString = "".to_string().white();
        let mut away_team: ColoredString = "".to_string().white();
        let mut home_score: ColoredString = "".to_string().white();
        let mut away_score: ColoredString = "".to_string().white();
        let mut home_rank: i64 = 0;
        let mut away_rank: i64 = 0;
        for comps in e.competitions {
            for c in comps.competitors {
                if c.home_away == "home" {
                    home_rank = c.curated_rank.current;
                    if let Some(true) = c.winner {
                        home_team = c.team.location.green();
                        home_score = c.score.green()
                    } else {
                        home_team = c.team.location.white();
                        home_score = c.score.white()
                    }
                } else if c.home_away == "away" {
                    away_rank = c.curated_rank.current;
                    if let Some(true) = c.winner {
                        away_team = c.team.location.green();
                        away_score = c.score.green()
                    } else {
                        away_team = c.team.location.white();
                        away_score = c.score.white();
                    }
                }
            }
            //println!("{:<12} on {}", e.short_name, d.format("%I:%M %p"));
        }
        sorted_games.push(Event {
            home_team,
            away_team,
            home_score,
            away_score,
            home_rank,
            away_rank,
            date: d,
        });
    }
    sorted_games.sort_by(|a, b| a.date.cmp(&b.date));
    for g in sorted_games {
        println!(
            "{:<18}({:>2}) @ {:<15}({:>2}) @ {:<5} : {} - {}",
            g.away_team,
            g.away_rank,
            g.home_team,
            g.home_rank,
            g.date.format("%I:%M %p"),
            g.away_score,
            g.home_score
        )
    }
}