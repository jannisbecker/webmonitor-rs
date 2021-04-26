use std::env;

use futures::StreamExt;

use log::info;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    options::{ClientOptions, FindOneOptions, ResolverConfig},
    Client, Collection, Database,
};

use crate::{
    error::DatabaseError,
    model::{InsertableJob, InsertableSnapshot, Job, Snapshot},
};

pub struct DatabaseAdapter {
    client: Client,
    database: Database,
    job_collection: Collection,
    snapshot_collection: Collection,
}

type Result<T> = core::result::Result<T, DatabaseError>;

impl DatabaseAdapter {
    pub async fn init() -> Result<Self> {
        let client_uri = env::var("DATABASE_URI")
            .expect("Please supply a valid DATABASE_URI in your .env file.");
        let database_name = env::var("DATABASE_NAME")
            .expect("Please supply a valid DATABASE_NAME in your .env file.");

        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;

        let client = Client::with_options(options)?;
        let database = client.database(database_name.as_str());
        let job_collection = database.collection("jobs");
        let snapshot_collection = database.collection("snapshots");
        info!("Connected to database.");

        Ok(Self {
            client,
            database,
            job_collection,
            snapshot_collection,
        })
    }

    pub async fn jobs_get_all(&self) -> Result<Vec<Job>> {
        let mut cursor = self.job_collection.find(None, None).await?;
        let mut jobs: Vec<Job> = Vec::new();

        while let Some(doc) = cursor.next().await {
            jobs.push(bson::from_document(doc?)?);
        }

        Ok(jobs)
    }

    pub async fn jobs_get_one(&self, id: &str) -> Result<Option<Job>> {
        let filter = doc! { "_id": ObjectId::with_string(id)? };
        let option = self.job_collection.find_one(filter, None).await?;

        match option {
            Some(doc) => Ok(Some(bson::from_document(doc)?)),
            None => Ok(None),
        }
    }

    pub async fn jobs_add(&self, job: InsertableJob) -> Result<Job> {
        let doc = bson::to_document(&job)?;

        let result = self.job_collection.insert_one(doc, None).await?;
        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        Ok(Job {
            id: id,
            name: job.name,
            url: job.url,
            show_diff: job.show_diff,
            interval: job.interval,
            filters: job.filters,
            notifiers: job.notifiers,
        })
    }

    pub async fn jobs_update(&self, job: Job) -> Result<Job> {
        let filter = doc! { "_id": &job.id.as_str() };
        let doc = bson::to_document(&job)?;

        self.job_collection.update_one(filter, doc, None).await?;

        Ok(job)
    }

    pub async fn jobs_delete(&self, id: &str) -> Result<()> {
        let filter = doc! { "_id": ObjectId::with_string(id)? };

        self.job_collection.delete_one(filter, None).await?;

        Ok(())
    }

    pub async fn snapshots_get_all(&self, job_id: &str) -> Result<Vec<Snapshot>> {
        let filter = doc! { "job_id": job_id };

        let mut cursor = self.snapshot_collection.find(filter, None).await?;

        let mut snapshots: Vec<Snapshot> = Vec::new();
        while let Some(doc) = cursor.next().await {
            snapshots.push(bson::from_document(doc?)?);
        }

        Ok(snapshots)
    }

    pub async fn snapshots_get_latest(&self, job_id: &str) -> Result<Option<Snapshot>> {
        let filter = doc! { "job_id": job_id };
        let options = FindOneOptions::builder().sort(doc! { "_id": -1}).build();

        let option = self.snapshot_collection.find_one(filter, options).await?;
        match option {
            Some(doc) => Ok(Some(bson::from_document(doc)?)),
            None => Ok(None),
        }
    }

    pub async fn snapshots_add(&self, snapshot: InsertableSnapshot) -> Result<Snapshot> {
        let doc = bson::to_document(&snapshot)?;

        let result = self.snapshot_collection.insert_one(doc, None).await?;
        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        Ok(Snapshot {
            id: id,
            job_id: snapshot.job_id,
            data: snapshot.data,
        })
    }

    pub async fn snapshots_get_one(&self, id: &str) -> Result<Option<Snapshot>> {
        let filter = doc! { "_id": ObjectId::with_string(id)? };

        let option = self.snapshot_collection.find_one(filter, None).await?;
        match option {
            Some(doc) => Ok(Some(bson::from_document(doc)?)),
            None => Ok(None),
        }
    }

    pub async fn snapshots_delete(&self, id: &str) -> Result<()> {
        let filter = doc! { "_id": ObjectId::with_string(id)? };

        self.job_collection.delete_one(filter, None).await?;

        Ok(())
    }
}
