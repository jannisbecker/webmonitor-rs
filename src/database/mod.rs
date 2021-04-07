use crate::model::{Job, Snapshot};
pub trait DataSource {
    fn new() -> Self;

    fn jobs_get_all(&self) -> Vec<Job>;
    fn jobs_get_one(&self, id: &str) -> Job;
    fn jobs_add(&self, job: Job) -> Job;
    fn jobs_update(&self, job: Job) -> Job;
    fn jobs_delete(&self, id: &str) -> Result<(), ()>;

    fn snapshots_get_all(&self, job_id: &str) -> Vec<Snapshot>;
    fn snapshots_get_latest(&self, job_id: &str) -> Snapshot;
    fn snapshots_get_one(&self, id: &str) -> Snapshot;
    fn snapshots_delete(&self, id: &str) -> Result<(), ()>;
}

#[cfg(feature = "use-mongodb")]
pub use mongo::DatabaseAdapter;
#[cfg(feature = "use-mongodb")]
mod mongo;

#[cfg(not(feature = "use-mongodb"))]
pub use sqlite::DatabaseAdapter;
#[cfg(not(feature = "use-mongodb"))]
mod sqlite;
