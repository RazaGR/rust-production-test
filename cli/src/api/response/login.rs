use serde::Serialize;

use super::error::Status;

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
}
