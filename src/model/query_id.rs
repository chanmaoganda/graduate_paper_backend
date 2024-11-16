use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct QueryById {
    #[serde(rename = "id")]
    pub inner: String,
}