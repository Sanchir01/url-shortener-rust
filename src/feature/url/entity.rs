use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug,Deserialize,Serialize,Validate)]
pub struct CreateUrlDTO {
    #[validate(url)]
    pub url: String,
}