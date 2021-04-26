use futures::future;
use std::error::Error;
use std::sync::Arc;

use webmonitor_core::{
    model::{CSSFilterOptions, DiscordNotifierOptions, Filter, InsertableJob, Notifier},
    monitoring::WebsiteMonitor,
    notifications::NotificationDispatcher,
    repository::Repository,
    scheduling::JobScheduler,
};

use dotenv::dotenv;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let repository = Arc::new(Repository::init().await?);
    let notification_dispatcher = Arc::new(NotificationDispatcher::new());
    let website_monitor = Arc::new(WebsiteMonitor::new(
        Arc::clone(&repository),
        Arc::clone(&notification_dispatcher),
    ));
    let job_scheduler = Arc::new(JobScheduler::new(Arc::clone(&website_monitor)));

    let job = InsertableJob {
        name: String::from("Check time every 10 seconds"),
        url: String::from("https://www.unixtimestamp.com/"),
        interval: 10,
        show_diff: true,

        filters: vec![
            Filter::CSSFilter(CSSFilterOptions {
                selector: String::from("div.ui.statistic"),
            }),
        ],
        notifiers: vec![Notifier::Discord(DiscordNotifierOptions {
            webhook_url: String::from("https://discord.com/api/webhooks/834762172088451078/9bO6xDtn2t7auMF8q184qIqvTzBYeYJYJl0B2ODhoNUobQ-VSiXJL9r476SwVQCtjEAS"),
            user_mentions: Some(String::from("@here, <@148892877253115904>")),
        })]
    };

    let added_job = repository.jobs_add(job).await?;
    let result = repository.jobs_get_one(added_job.id.as_str()).await?;

    info!("Scheduling existing jobs");
    future::join_all(
        repository
            .jobs_get_all()
            .await?
            .into_iter()
            .map(|job| job_scheduler.schedule(job)),
    )
    .await;

    loop {}

    Ok(())
}
