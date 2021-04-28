use mongodb::bson;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WebmonitorError>;

#[derive(Error, Debug)]
pub enum WebmonitorError {
    #[error("Error while parsing the given Mongodb ObjectID")]
    OIDParseError(#[from] bson::oid::Error),

    #[error("Error while serializing the given struct to a Mongodb document")]
    BsonSerializeError(#[from] bson::ser::Error),

    #[error("Error while deserializing the given Mongodb document to it's struct form")]
    BsonDeserializeError(#[from] bson::de::Error),

    #[error("Error while accessing the Mongodb database")]
    MongoDBError(#[from] mongodb::error::Error),

    #[error("Error while requesting web data")]
    RequestError(#[from] reqwest::Error),

    #[error("Error while parsing the given CSS selector")]
    SelectorParseError,
}
