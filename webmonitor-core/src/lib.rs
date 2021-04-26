pub mod error;
pub mod model;
pub mod monitoring;
pub mod notifications;
pub mod repository;
pub mod scheduling;

pub struct Webmonitor;

impl Webmonitor {
    async fn init();

    async fn create_job();

    async fn edit_job();

    async fn delete_job();
}
