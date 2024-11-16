use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryById {
    #[serde(rename = "id")]
    pub inner: String,
}