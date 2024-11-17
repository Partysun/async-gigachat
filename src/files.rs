use crate::client::Client;
use crate::result::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

use reqwest::multipart::{Form, Part};

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

// Trait to allow a file_path as &str or entire file as bytes
pub trait FileSource {
    fn create_form_part(&self, name: &str) -> Result<Part>;
}

// Implementation for file paths
impl FileSource for &str {
    fn create_form_part(&self, name: &str) -> Result<Part> {
        let mut file_handle = File::open(self)?;
        let mut buffer = Vec::new();
        file_handle.read_to_end(&mut buffer)?;
        let part = Part::bytes(buffer)
            .file_name(name.to_string())
            .mime_str("image/png")?;
        Ok(part)
    }
}

// Implementation for byte slices
impl FileSource for &[u8] {
    fn create_form_part(&self, name: &str) -> Result<Part> {
        let part = Part::bytes(self.to_vec())
            .file_name(name.to_string())
            .mime_str("image/png")?;
        Ok(part)
    }
}

fn create_file_form<T: FileSource>(source: &T, name: &str) -> Result<Form> {
    let part = source.create_form_part(name)?;
    Ok(Form::new().part("file", part))
}

// Create using file path. Easier, than you think...
//
// let response = Files::new(client.clone())
//     .create_file(
//         &"./examples/image_file_sample.png",
//         "image_file_sample.png".to_string(),
//     )
//     .await?;
//
//TODO: need to implement more mime_str types. Not only image/png
impl Files {
    pub fn new(client: Client) -> Self {
        Files { client }
    }

    // https://developers.sber.ru/docs/ru/gigachat/api/working-with-files?ext=image
    pub async fn create_file<T: FileSource>(
        self,
        source: &T,
        file_name: String,
    ) -> Result<FileResponse> {
        let form = create_file_form(source, file_name.as_str())?;
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
