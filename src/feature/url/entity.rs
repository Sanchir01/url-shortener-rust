use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate,ValidationError};
fn validate_not_nil(id: &Uuid) -> Result<(), ValidationError> {
    if *id == Uuid::nil() {
        Err(ValidationError::new("id_cant_be_nil"))
    } else {
        Ok(())
    }
}
#[derive(Debug,Deserialize,Serialize,Validate)]
pub struct CreateUrlDTO {
    #[validate(url)]
    pub url: String,
}

#[derive(Debug,Deserialize,Serialize,Validate)]
pub struct DeleteUrlDto{
    #[validate(custom(function  = "validate_not_nil") )]
    pub id:Uuid
}