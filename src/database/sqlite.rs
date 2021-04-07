use super::DataSource;
use crate::model::{Job, Snapshot};

pub struct DatabaseAdapter;

impl DataSource for DatabaseAdapter {
    fn jobs_get_all(&self) -> Vec<Job> {
        todo!()
    }

    fn jobs_get_one(&self, id: &str) -> Job {
        todo!()
    }

    fn jobs_add(&self, job: Job) -> Job {
        todo!()
    }

    fn jobs_update(&self, job: Job) -> Job {
        todo!()
    }

    fn jobs_delete(&self, job: Job) -> Result<(), ()> {
        todo!()
    }

    fn snapshots_get_all(&self, job_id: &str) -> Vec<Snapshot> {
        todo!()
    }

    fn snapshots_get_latest(&self, job_id: &str) -> Snapshot {
        todo!()
    }

    fn snapshots_get_one(&self, id: &str) -> Snapshot {
        todo!()
    }

    fn snapshots_delete(&self, id: &str) -> Result<(), ()> {
        todo!()
    }
}
