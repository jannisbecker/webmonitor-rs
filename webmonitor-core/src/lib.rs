pub mod database;
pub mod error;
pub mod model;
pub mod notifier;
pub mod scheduler;
pub mod watcher;

pub struct Webmonitor;

impl Webmonitor {
    async fn init();

    async fn create_job();

    async fn edit_job();

    async fn delete_job();
}
