use mongodb::bson;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Couldn't parse given ObjectID")]
    OIDParseError(#[from] bson::oid::Error),

    #[error("Couldn't serialize given struct")]
    BsonSerializeError(#[from] bson::ser::Error),

    #[error("Couldn't deserialize given document")]
    BsonDeserializeError(#[from] bson::de::Error),

    #[error("MongoDB has thrown an error")]
    MongoDBError(#[from] mongodb::error::Error),
}

#[derive(Error, Debug)]
pub enum WatcherError {
    #[error("Couldn't reach the website to be watched")]
    RequestError(#[from] reqwest::Error),

    #[error("Couldn't parse the given Filter selector")]
    SelectorParseError(#[from] cssparser::ParseError),
}
