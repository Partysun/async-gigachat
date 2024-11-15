use crate::client::Client;
use serde::{Deserialize, Serialize};
// use crate::errors::GigaChatError;
use crate::result::Result;
use std::{fs::File, io::Read};

use reqwest::multipart::{Form, Part};

#[derive(Debug)]
pub struct FilesRequest {
    pub file_name: String,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResponse {
    pub id: String,
    pub bytes: i32,
    pub created_at: u64,
    pub filename: String,
    pub object: String,
    pub purpose: String,
    pub access_policy: String,
}

#[derive(Deserialize, Debug)]
pub struct FilesResponse {
    pub data: Vec<FileResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDeletionResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

pub struct Files {
    client: Client,
}

impl Files {
    pub fn new(client: Client) -> Self {
        Files { client }
    }

    // https://developers.sber.ru/docs/ru/gigachat/api/working-with-files?ext=image
    pub async fn create_files(self, request: FilesRequest) -> Result<FileResponse> {
        let mut file_handle = File::open(request.file_path).unwrap();
        let mut data = Vec::new();
        file_handle.read_to_end(&mut data).unwrap();
        let part = Part::bytes(data)
            .file_name(request.file_name)
            .mime_str("image/png")
            .unwrap();
        println!("{:?}", part);
        let form = Form::new().part("file", part);
        let response = self.client.post_with_form("/files", form).await?;

        Ok(response)
    }

    pub async fn get_files(self) -> Result<FilesResponse> {
        let response = self.client.get("/files").await?;

        Ok(response)
    }

    pub async fn get_file_by_id(self, file: String) -> Result<FileResponse> {
        let path = format!("/files/{}", file);
        let response = self.client.get(&path).await?;

        Ok(response)
    }

    pub async fn delete_file_by_id(self, file: String) -> Result<FileDeletionResponse> {
        let path = format!("/files/{}/delete", file);
        let response = self.client.post(&path, "{}").await?;

        Ok(response)
    }
}
