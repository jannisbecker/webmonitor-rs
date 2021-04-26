use std::{collections::HashSet, sync::Arc, time::Duration};

use log::{info, warn};
use tokio::{sync::RwLock, time};

use crate::{model::Job, monitoring::WebsiteMonitor};

pub struct JobScheduler {
    watcher: Arc<WebsiteMonitor>,
    scheduled_jobs: Arc<RwLock<HashSet<String>>>,
}

impl JobScheduler {
    pub fn new(watcher: Arc<WebsiteMonitor>) -> Self {
        Self {
            watcher,
            scheduled_jobs: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub async fn schedule(&self, job: Job) {
        let jobs_ref = Arc::clone(&self.scheduled_jobs);
        let watcher_ref = Arc::clone(&self.watcher);

        {
            let mut scheduled_jobs = jobs_ref.write().await;
            scheduled_jobs.insert(job.id.clone());
        }

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(job.interval));

            loop {
                interval.tick().await;

                let scheduled_jobs = jobs_ref.read().await;
                if !scheduled_jobs.contains(&job.id) {
                    break;
                }

                let result = watcher_ref.run_website_check_for_job(&job).await;

                if let Err(e) = result {
                    warn!("There was a problem checking job '{}': {}", &job.name, e);
                }
            }
        });
    }

    pub async fn unschedule(&self, job_id: &str) {
        let mut scheduled_jobs = self.scheduled_jobs.write().await;
        scheduled_jobs.remove(job_id);
    }
}
