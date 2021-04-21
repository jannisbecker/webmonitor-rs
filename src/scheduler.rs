use std::{collections::HashSet, sync::Arc, time::Duration};

use tokio::{sync::RwLock, time};

use crate::{model::Job, watcher::Watcher};

pub struct Scheduler {
    watcher: Arc<Watcher>,
    scheduled_jobs: Arc<RwLock<HashSet<String>>>,
}

impl Scheduler {
    pub fn new(watcher: Arc<Watcher>) -> Self {
        Self {
            watcher,
            scheduled_jobs: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub async fn schedule(&self, job: Job) {
        let jobs_ref = Arc::clone(&self.scheduled_jobs);
        let watcher_ref = Arc::clone(&self.watcher);

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(job.interval));

            loop {
                interval.tick().await;

                let scheduled_jobs = jobs_ref.read().await;
                if !scheduled_jobs.contains(&job.id) {
                    break;
                }

                match watcher_ref.run_watcher_for_job(&job).await {
                    Ok(()) => (),
                    // TODO error handling on failed jobs
                    WatcherError => (),
                };
            }
        });
    }

    pub async fn unschedule(&self, job_id: &str) {
        let mut scheduled_jobs = self.scheduled_jobs.write().await;

        scheduled_jobs.remove(job_id);
    }
}
