use std::{collections::HashSet, sync::Arc, time::Duration};

use tokio::{sync::RwLock, time};

use crate::model::Job;

pub struct Scheduler {
    scheduled_jobs: Arc<RwLock<HashSet<String>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            scheduled_jobs: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub async fn schedule(&self, job: &'static Job) {
        let jobs_ref = Arc::clone(&self.scheduled_jobs);

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(job.interval));

            loop {
                interval.tick().await;

                let scheduled_jobs = jobs_ref.read().await;

                if !scheduled_jobs.contains(&job.id) {
                    break;
                }

                // do stuff
            }
        });
    }

    pub async fn unschedule(&self, job_id: &str) {
        let mut scheduled_jobs = self.scheduled_jobs.write().await;

        scheduled_jobs.remove(job_id);
    }
}
