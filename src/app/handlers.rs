use std::sync::Arc;
use crate::app::services::Services;
use crate::feature::url::handler::UrlHandler;

pub struct Handlers {
    pub url_handler: Arc<UrlHandler>
}
impl Handlers {
    pub fn new(services: Arc<Services>) -> Self {
        Self { url_handler: Arc::new(UrlHandler::new_handler(services.url_service.clone())) }
    }
}