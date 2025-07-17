use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::feature::url::repository::UrlRepository;

#[derive(Clone)]
pub struct Repositories {
    pub url_repository: Arc<UrlRepository>,
}

impl Repositories {
    pub fn new(chouse: Pool<Postgres>) -> Self {
        Self {
            url_repository:Arc::new(UrlRepository::new_url_repository(chouse.clone())),
        }
    }
}
