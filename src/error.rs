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
