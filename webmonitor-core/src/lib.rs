use std::sync::Arc;

use futures::future;
use model::{InsertableJob, Job};
use monitoring::WebsiteMonitor;
use repository::Repository;
use scheduling::JobScheduler;

use crate::error::Result;

pub mod error;
pub mod filters;
pub mod model;
pub mod monitoring;
pub mod notifications;
pub mod repository;
pub mod scheduling;

pub struct Webmonitor {
    repository: Arc<Repository>,
    monitor: Arc<WebsiteMonitor>,
    scheduler: Arc<JobScheduler>,
}

impl Webmonitor {
    pub async fn init() -> Result<Self> {
        let repository = Arc::new(Repository::init().await?);
        let monitor = Arc::new(WebsiteMonitor::new(Arc::clone(&repository)));
        let scheduler = Arc::new(JobScheduler::new(Arc::clone(&monitor)));

        future::join_all(
            repository
                .jobs_get_all()
                .await?
                .into_iter()
                .map(|job| scheduler.schedule(job)),
        )
        .await;

        Ok(Self {
            repository,
            monitor,
            scheduler,
        })
    }

    pub async fn add_job(&self, job: InsertableJob) -> Result<Job> {
        self.repository.jobs_add(job).await
    }

    pub async fn get_job(&self, job_id: &str) -> Result<Option<Job>> {
        self.repository.jobs_get_one(job_id).await
    }
}
