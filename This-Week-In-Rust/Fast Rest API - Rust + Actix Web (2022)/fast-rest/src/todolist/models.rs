use serde::Deserialize;

/* define what we expect in our post */
#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String,
}
