use std::sync::Arc;
use crate::app::repositories::Repositories;
use crate::feature::url::service::UrlService;

#[derive(Clone)]
pub struct Services {
    pub url_service: Arc<UrlService>,
}

impl Services{
    pub fn new(repo: Arc<Repositories>)->Self{
        Self{
            url_service: Arc::new(UrlService::new(
                repo.url_repository.clone(),
            ))
        }
    }
}