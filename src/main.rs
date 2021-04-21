use std::error::Error;
use std::sync::Arc;

use dotenv::dotenv;

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

    let db = Arc::new(DatabaseAdapter::init().await?);
    let watcher = Arc::new(Watcher::new(Arc::clone(&db)));
    let scheduler = Scheduler::new(watcher);

    let _ = db
        .jobs_get_all()
        .await?
        .into_iter()
        .map(|job| scheduler.schedule(job));

    let job = InsertableJob {
        name: String::from("bruh"),
        url: String::from("https://monsterhunterfor20bucks.com"),
        interval: 20,
        filters: vec![Filter::CSSFilter(CSSFilterOptions {
            selector: String::from("bruh selector"),
        })],
    };

    let added_job = db.jobs_add(job).await?;
    let result = db.jobs_get_one(added_job.id.as_str()).await?;

    Ok(())
}
