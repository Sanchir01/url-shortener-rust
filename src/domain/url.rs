use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow,Serialize,Deserialize)]
pub struct Url {
    pub id: Uuid,
    pub alias: String,
    pub url: String,
}
