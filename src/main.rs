use futures::future;
use notifier::Notifier;
use std::error::Error;
use std::sync::Arc;

use dotenv::dotenv;
use log::info;

use database::DatabaseAdapter;
use scheduler::Scheduler;
use watcher::Watcher;

mod database;
mod error;
mod model;
mod notifier;
mod scheduler;
mod watcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let db = Arc::new(DatabaseAdapter::init().await?);
    let notifier = Arc::new(Notifier::new());
    let watcher = Arc::new(Watcher::new(Arc::clone(&db), Arc::clone(&notifier)));
    let scheduler = Arc::new(Scheduler::new(Arc::clone(&watcher)));

    info!("Scheduling existing jobs");
    future::join_all(
        db.jobs_get_all()
            .await?
            .into_iter()
            .map(|job| scheduler.schedule(job)),
    )
    .await;

    loop {}

    Ok(())
}
