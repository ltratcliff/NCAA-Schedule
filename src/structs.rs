use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub events: Vec<Event>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub date: String,
    pub name: String,
    pub competitions: Vec<Competition>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub competitors: Vec<Competitor>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competitor {
    pub home_away: String,
    pub winner: Option<bool>,
    pub team: Team,
    pub score: String,
    pub curated_rank: CuratedRank,

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub location: String,
    pub name: String,
    pub abbreviation: String,
    pub display_name: String,
    pub short_display_name: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CuratedRank {
    pub current: i64,
}
