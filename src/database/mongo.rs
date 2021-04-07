use super::DataSource;
use crate::model::{Job, Snapshot};
use mongodb::{
    bson,
    bson::doc,
    options::FindOneOptions,
    sync::{Client, Collection, Database},
};

pub struct DatabaseAdapter {
    client: Client,
    database: Database,
    job_collection: Collection,
    snapshot_collection: Collection,
}

impl DataSource for DatabaseAdapter {
    fn new() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
        let database = client.database("webmonitor");
        let job_collection = database.collection("jobs");
        let snapshot_collection = database.collection("snapshots");

        DatabaseAdapter {
            client,
            database,
            job_collection,
            snapshot_collection,
        }
    }

    fn jobs_get_all(&self) -> Vec<Job> {
        self.job_collection
            .find(None, None)
            .unwrap()
            .into_iter()
            .map(|res| {
                let doc = res.unwrap();
                bson::from_document(doc).unwrap()
            })
            .collect()
    }

    fn jobs_get_one(&self, id: &str) -> Job {
        let filter = doc! { "id": id };

        let doc = self.job_collection.find_one(filter, None).unwrap().unwrap();

        bson::from_document(doc).unwrap()
    }

    fn jobs_add(&self, job: Job) -> Job {
        let doc = bson::to_document(&job).unwrap();
        self.job_collection.insert_one(doc, None).unwrap();

        job
    }

    fn jobs_update(&self, job: Job) -> Job {
        let doc = bson::to_document(&job).unwrap();
        let filter = doc! { "id": &job.id };

        self.job_collection.update_one(filter, doc, None).unwrap();

        job
    }

    fn jobs_delete(&self, id: &str) -> Result<(), ()> {
        let filter = doc! { "id": id };

        self.job_collection.delete_one(filter, None).unwrap();

        Ok(())
    }

    fn snapshots_get_all(&self, job_id: &str) -> Vec<Snapshot> {
        let filter = doc! { "job_id": job_id };

        self.snapshot_collection
            .find(filter, None)
            .unwrap()
            .into_iter()
            .map(|res| {
                let doc = res.unwrap();
                bson::from_document(doc).unwrap()
            })
            .collect()
    }

    fn snapshots_get_latest(&self, job_id: &str) -> Snapshot {
        let filter = doc! { "job_id": job_id };

        let doc = self
            .snapshot_collection
            .find_one(filter, None)
            .unwrap()
            .unwrap();

        bson::from_document(doc).unwrap()
    }

    fn snapshots_get_one(&self, id: &str) -> Snapshot {
        let filter = doc! { "id": id };
        let options = FindOneOptions::builder().sort(doc! { "_id": 1}).build();

        let doc = self
            .snapshot_collection
            .find_one(filter, options)
            .unwrap()
            .unwrap();

        bson::from_document(doc).unwrap()
    }

    fn snapshots_delete(&self, id: &str) -> Result<(), ()> {
        let filter = doc! { "id": id };

        self.job_collection.delete_one(filter, None).unwrap();

        Ok(())
    }
}
