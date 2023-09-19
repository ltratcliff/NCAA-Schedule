use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub leagues: Vec<League>,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub uid: String,
    pub name: String,
    pub abbreviation: String,
    pub midsize_name: String,
    pub slug: String,
    pub season: Season,
    pub logos: Vec<Logo>,
    pub calendar_type: String,
    pub calendar_is_whitelist: bool,
    pub calendar_start_date: String,
    pub calendar_end_date: String,
    pub calendar: Vec<Calendar>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub year: i64,
    pub start_date: String,
    pub end_date: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub type_field: Type,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub name: String,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logo {
    pub href: String,
    pub width: i64,
    pub height: i64,
    pub alt: String,
    pub rel: Vec<String>,
    pub last_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub label: String,
    pub value: String,
    pub start_date: String,
    pub end_date: String,
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub label: String,
    pub alternate_label: String,
    pub detail: String,
    pub value: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub uid: String,
    pub date: String,
    pub name: String,
    pub short_name: String,
    pub season: Season2,
    pub week: Week,
    pub competitions: Vec<Competition>,
    pub links: Vec<Link3>,
    pub status: Status2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season2 {
    pub year: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub id: String,
    pub uid: String,
    pub date: String,
    pub attendance: i64,
    #[serde(rename = "type")]
    pub type_field: Type2,
    pub time_valid: bool,
    pub neutral_site: bool,
    pub conference_competition: bool,
    pub play_by_play_available: bool,
    pub recent: bool,
    pub venue: Venue,
    pub competitors: Vec<Competitor>,
    pub notes: Vec<Value>,
    pub status: Status,
    pub broadcasts: Vec<Broadcast>,
    pub leaders: Vec<Leader>,
    pub groups: Option<Groups>,
    pub format: Format,
    pub start_date: String,
    pub geo_broadcasts: Vec<GeoBroadcast>,
    #[serde(default)]
    pub headlines: Vec<Headline>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type2 {
    pub id: String,
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: String,
    pub full_name: String,
    pub address: Address,
    pub capacity: i64,
    pub indoor: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competitor {
    pub id: String,
    pub uid: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub order: i64,
    pub home_away: String,
    pub winner: Option<bool>,
    pub team: Team,
    pub score: String,
    pub linescores: Option<Vec<Linescore>>,
    pub statistics: Vec<Value>,
    pub curated_rank: CuratedRank,
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub uid: String,
    pub location: String,
    pub name: String,
    pub abbreviation: String,
    pub display_name: String,
    pub short_display_name: String,
    pub color: Option<String>,
    pub alternate_color: Option<String>,
    pub is_active: bool,
    pub venue: Venue2,
    pub links: Vec<Link>,
    pub logo: String,
    pub conference_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue2 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub rel: Vec<String>,
    pub href: String,
    pub text: String,
    pub is_external: bool,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linescore {
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CuratedRank {
    pub current: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub name: String,
    pub abbreviation: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub summary: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub clock: f64,
    pub display_clock: String,
    pub period: i64,
    #[serde(rename = "type")]
    pub type_field: Type3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type3 {
    pub id: String,
    pub name: String,
    pub state: String,
    pub completed: bool,
    pub description: String,
    pub detail: String,
    pub short_detail: String,
    pub alt_detail: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcast {
    pub market: String,
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader {
    pub name: String,
    pub display_name: String,
    pub short_display_name: String,
    pub abbreviation: String,
    pub leaders: Vec<Leader2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader2 {
    pub display_value: String,
    pub value: f64,
    pub athlete: Athlete,
    pub team: Team3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Athlete {
    pub id: String,
    pub full_name: String,
    pub display_name: String,
    pub short_name: String,
    pub links: Vec<Link2>,
    pub headshot: Option<String>,
    pub jersey: String,
    pub position: Position,
    pub team: Team2,
    pub active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link2 {
    pub rel: Vec<String>,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team2 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team3 {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Groups {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub is_conference: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub regulation: Regulation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regulation {
    pub periods: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoBroadcast {
    #[serde(rename = "type")]
    pub type_field: Type4,
    pub market: Market,
    pub media: Media,
    pub lang: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type4 {
    pub id: String,
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub short_link_text: String,
    #[serde(default)]
    pub video: Vec<Video>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: i64,
    pub source: String,
    pub headline: String,
    pub thumbnail: String,
    pub duration: i64,
    pub tracking: Tracking,
    pub device_restrictions: DeviceRestrictions,
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracking {
    pub sport_name: String,
    pub league_name: String,
    pub coverage_type: String,
    pub tracking_name: String,
    pub tracking_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRestrictions {
    #[serde(rename = "type")]
    pub type_field: String,
    pub devices: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub api: Api,
    pub web: Web,
    pub source: Source,
    pub mobile: Mobile,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    #[serde(rename = "self")]
    pub self_field: Self_field,
    pub artwork: Artwork,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artwork {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web {
    pub href: String,
    pub short: Short,
    #[serde(rename = "self")]
    pub self_field: Self_field2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Short {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub mezzanine: Mezzanine,
    pub flash: Flash,
    pub hds: Hds,
    #[serde(rename = "HLS")]
    pub hls: Hls,
    #[serde(rename = "HD")]
    pub hd: Hd2,
    pub full: Full,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mezzanine {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flash {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hds {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hls {
    pub href: String,
    #[serde(rename = "HD")]
    pub hd: Hd,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hd {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hd2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Full {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mobile {
    pub alert: Alert,
    pub source: Source2,
    pub href: String,
    pub streaming: Streaming,
    pub progressive_download: ProgressiveDownload,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source2 {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Streaming {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressiveDownload {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link3 {
    pub language: Option<String>,
    pub rel: Vec<String>,
    pub href: String,
    pub text: String,
    pub short_text: String,
    pub is_external: bool,
    pub is_premium: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status2 {
    pub clock: f64,
    pub display_clock: String,
    pub period: i64,
    #[serde(rename = "type")]
    pub type_field: Type5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type5 {
    pub id: String,
    pub name: String,
    pub state: String,
    pub completed: bool,
    pub description: String,
    pub detail: String,
    pub short_detail: String,
    pub alt_detail: Option<String>,
}
