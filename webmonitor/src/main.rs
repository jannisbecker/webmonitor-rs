use std::error::Error;

use webmonitor_core::{
    model::{CSSFilterOptions, DiscordNotificationOptions, Filter, InsertableJob, Notification},
    Webmonitor,
};

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let monitor = Webmonitor::init().await?;

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
        notifications: vec![Notification::Discord(DiscordNotificationOptions {
            webhook_url: String::from("https://discord.com/api/webhooks/834762172088451078/9bO6xDtn2t7auMF8q184qIqvTzBYeYJYJl0B2ODhoNUobQ-VSiXJL9r476SwVQCtjEAS"),
            user_mentions: Some(String::from("@here, <@148892877253115904>")),
        })]
    };

    let added_job = &monitor.add_job(job).await?;
    let _ = &monitor.get_job(added_job.id.as_str()).await?;

    loop {}

    Ok(())
}
