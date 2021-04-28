use bson::serde_helpers::{hex_string_as_object_id, serialize_u64_as_i64};
use mongodb::bson::oid::ObjectId;
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
    #[serde(rename = "_id", with = "hex_string_as_object_id")]
    pub id: String,

    pub name: String,
    pub url: String,
    pub show_diff: bool,

    #[serde(serialize_with = "serialize_u64_as_i64")]
    pub interval: u64,
    pub filters: Vec<Filter>,
    pub notifications: Vec<Notification>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InsertableJob {
    pub name: String,
    pub url: String,
    pub show_diff: bool,

    #[serde(serialize_with = "serialize_u64_as_i64")]
    pub interval: u64,
    pub filters: Vec<Filter>,
    pub notifications: Vec<Notification>,
}

// Snapshots of a Job
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(rename = "_id", with = "hex_string_as_object_id")]
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
pub enum Notification {
    Discord(DiscordNotificationOptions),
    Email(EmailNotificationOptions),
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordNotificationOptions {
    pub webhook_url: String,
    pub user_mentions: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmailNotificationOptions {
    pub sender: String,
    pub recipient: String,
    pub subject: String,
}
