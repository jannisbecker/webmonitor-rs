use futures::future;
use model::{
    CSSFilterOptions, DiscordNotifierOptions, Filter, InsertableJob, Notifier as NotifierModel,
};
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

    // let job = InsertableJob {
    //     name: String::from("Check time every 10 seconds"),
    //     url: String::from("https://time.is/"),
    //     interval: 10,
    //     filters: vec![Filter::CSSFilter(CSSFilterOptions {
    //         selector: String::from("div#clock0_bg"),
    //     })],
    //     notifiers: vec![NotifierModel::Discord(DiscordNotifierOptions {
    //         webhook_url: String::from("https://discord.com/api/webhooks/834762172088451078/9bO6xDtn2t7auMF8q184qIqvTzBYeYJYJl0B2ODhoNUobQ-VSiXJL9r476SwVQCtjEAS")
    //     })]
    // };

    // let added_job = db.jobs_add(job).await?;
    // let result = db.jobs_get_one(added_job.id.as_str()).await?;

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
