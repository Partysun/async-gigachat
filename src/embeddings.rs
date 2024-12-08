use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::GigaChatError;
use crate::result::Result;

#[derive(Debug, Serialize, Builder)]
#[builder(setter(into, strip_option))]
pub struct EmbeddingsRequest {
    #[builder(default = "String::from(\"Embeddings\")")]
    pub model: String,
    pub input: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
}

#[derive(Debug, Deserialize)]
pub struct Points {
    pub object: String,
    pub embedding: Vec<f32>,
    pub usage: Usage,
    pub index: i32,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    pub object: String,
    pub data: Vec<Points>,
    pub model: String,
}

impl From<EmbeddingsRequestBuilderError> for GigaChatError {
    fn from(error: EmbeddingsRequestBuilderError) -> Self {
        GigaChatError::SystemError(error.to_string())
    }
}

pub struct Embeddings {
    client: Client,
}

impl Embeddings {
    pub fn new(client: Client) -> Self {
        Embeddings { client }
    }

    pub async fn encode(self, request: EmbeddingsRequest) -> Result<EmbeddingsResponse> {
        let response = self.client.post("/embeddings", request).await?;
        Ok(response)
    }
}
