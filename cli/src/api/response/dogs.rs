use entity::dog::Model;
use serde::Serialize;

use super::error::Status;

#[derive(Serialize)]
pub struct DogCreateResponse {
    pub status: Status,
    pub data: Option<Model>,
}
