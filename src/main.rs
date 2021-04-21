use futures::future;
use std::error::Error;
use std::sync::Arc;

use dotenv::dotenv;

use log::info;
use model::{CSSFilterOptions, Filter, InsertableJob};

mod database;
mod error;
mod model;
mod scheduler;
mod watcher;

use database::DatabaseAdapter;
use scheduler::Scheduler;
use watcher::Watcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let db = Arc::new(DatabaseAdapter::init().await?);
    let watcher = Arc::new(Watcher::new(Arc::clone(&db)));
    let scheduler = Arc::new(Scheduler::new(Arc::clone(&watcher)));

    let all_jobs = db.jobs_get_all().await?;

    future::join_all(all_jobs.into_iter().map(|job| scheduler.schedule(job))).await;

    info!("Scheduled existing jobs");

    loop {}

    Ok(())
}
