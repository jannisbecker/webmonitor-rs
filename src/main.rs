use model::{CSSFilterOptions, Filter, Job};
use mongodb::{bson, sync::Client};

mod database;
mod model;

use database::{DataSource, DatabaseAdapter};

fn main() {
    let job = Job {
        id: String::from("9ti8hu1209guwqe0=9ug"),
        name: String::from("bruh"),
        url: String::from("https://monsterhunterfor20bucks.com"),
        interval: 20,
        filters: vec![Filter::CSSFilter(CSSFilterOptions {
            selector: String::from("bruh selector"),
        })],
    };

    let db = DatabaseAdapter::new();

    println!("job struct: {:#?}", job);

    let job = db.jobs_add(job);

    let result = db.jobs_get_one(job.id.as_str());

    println!("job document after deserialize: {:#?}", result);
}
