//! # Webmonitor Core
//!
//! This is the core library crate for the Webmonitor tool,
//! a piece of software to monitor any number of webpages
//! as well as getting notified whenever they change (and what changed!)

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
    /// Creates a new instance of the Webmonitor service,
    /// and initializes all needed connections and services.
    /// Use this as the starting point of using the crate.
    ///
    /// # Examples
    ///
    /// ```
    /// let webmonitor = Webmonitor::init().await?;
    /// ```
    ///
    /// # Errors
    ///
    /// The initialization could fail if the database connection couldn't be established.
    /// In that case, it will return a WebmonitorError::MongoDBError.
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

    /// Adds a new job to the webmonitor. It will be immediately scheduled
    /// and checked according to its given options.
    /// It returns a Job struct of the inserted job (with its ID added),
    /// or fails if the Job couldn't be created or scheduled.
    ///
    /// # Examples
    ///
    /// ```
    /// let new_job = InsertableJob {
    ///     name: String::from("Check time every 10 seconds"),
    ///     url: String::from("https://www.unixtimestamp.com/"),
    ///     interval: 10,
    ///     show_diff: true,
    ///     filters: vec![
    ///         Filter::CSSFilter(CSSFilterOptions {
    ///             selector: String::from("div.ui.statistic"),
    ///         }),
    ///     ],
    ///     notifications: vec![]
    /// };
    ///
    /// let added_job = webmonitor.add_job(insert_job).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Adding a job can fail if either inserting the record into the database
    /// fails (for various reasons), or if scheduling the job fails.
    pub async fn add_job(&self, job: InsertableJob) -> Result<Job> {
        self.repository.jobs_add(job).await
    }

    /// Returns a single job struct with the given id, or none if
    /// a job with that id does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// let some_job = webmonitor.get_job("7aw98fa89wf789awf89a").await?;
    ///
    /// if let Some(job) = some_job {
    ///     println!(job.name);
    /// }
    /// ```
    ///
    /// # Errors
    /// Returning a job only fails if there's a problem with the database connection,
    /// or when parsing the database document into the Job struct.
    /// If the function doesn't find a Job with the given id, it will simply return None;
    pub async fn get_job(&self, job_id: &str) -> Result<Option<Job>> {
        self.repository.jobs_get_one(job_id).await
    }
}
