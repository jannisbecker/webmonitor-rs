use std::error::Error;

use dotenv::dotenv;

use model::{CSSFilterOptions, Filter, InsertableJob, Job, XPathFilterOptions};

mod database;
mod error;
mod model;

use database::DatabaseAdapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let job = InsertableJob {
        name: String::from("bruh"),
        url: String::from("https://monsterhunterfor20bucks.com"),
        interval: 20,
        filters: vec![Filter::CSSFilter(CSSFilterOptions {
            selector: String::from("bruh selector"),
        })],
    };

    let db = DatabaseAdapter::init().await?;

    let added_job = db.jobs_add(job).await?;

    println!("{:#?}", added_job);

    let result = db.jobs_get_one(added_job.id.as_str()).await?;

    println!("{:#?}", result);

    Ok(())
}

// let job = Job {
//     id: "6076db0200a72e9c00b94351".to_string(),
//     name: String::from("bruh"),
//     url: String::from("https://monsterhunterfor20bucks.com"),
//     interval: 20,
//     filters: vec![
//         Filter::CSSFilter(CSSFilterOptions {
//             selector: String::from("css selector"),
//         }),
//         Filter::XPathFilter(XPathFilterOptions {
//             selector: String::from("xpath selector"),
//         }),
//     ],
// };

// let doc = bson::to_document(&job)?;

// println!("{:#?}", doc);

// let deser: Job = bson::from_document(doc)?;

// println!("{:#?}", deser);
