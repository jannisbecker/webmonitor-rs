use mongodb::bson::{oid::ObjectId, serde_helpers::*};
use serde::{Deserialize, Deserializer, Serialize};

pub fn deserialize_object_id_to_hex_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let object_id = ObjectId::deserialize(deserializer)?;
    Ok(object_id.to_hex())
}

// Job structs
#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_hex_string_as_object_id",
        deserialize_with = "deserialize_object_id_to_hex_string"
    )]
    pub id: String,
    pub name: String,
    pub url: String,

    #[serde(serialize_with = "serialize_u64_as_i64")]
    pub interval: u64,
    pub filters: Vec<Filter>,
    pub notifiers: Vec<Notifier>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InsertableJob {
    pub name: String,
    pub url: String,

    #[serde(serialize_with = "serialize_u64_as_i64")]
    pub interval: u64,
    pub filters: Vec<Filter>,
    pub notifiers: Vec<Notifier>,
}

// Snapshots of a Job
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_hex_string_as_object_id",
        deserialize_with = "deserialize_object_id_to_hex_string"
    )]
    pub id: String,
    pub job_id: String,
    pub data: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InsertableSnapshot {
    pub job_id: String,
    pub data: String,
}

// Filters to apply to Jobs
#[derive(Clone, Serialize, Deserialize)]
pub enum Filter {
    CSSFilter(CSSFilterOptions),
    XPathFilter(XPathFilterOptions),
    Html2TextFilter,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CSSFilterOptions {
    pub selector: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XPathFilterOptions {
    pub selector: String,
}

// Notifiers to send out notifications for Jobs
#[derive(Clone, Serialize, Deserialize)]
pub enum Notifier {
    Discord(DiscordNotifierOptions),
    Email(EmailNotifierOptions),
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordNotifierOptions {
    pub webhook_url: String,
    pub user_mentions: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmailNotifierOptions {
    pub sender: String,
    pub recipient: String,
    pub subject: String,
}
