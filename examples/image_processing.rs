use async_gigachat::files::{Files, FilesRequest};
use async_gigachat::{client::Client, config::GigaChatConfig, result::Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::new();

    let client = Client::with_config(config);

    // let request = FilesRequest {
    //     file_path: "./examples/image_file_sample.png".to_string(),
    //     file_name: "image_file_sample.png".to_string(),
    // };

    // let response = Files::new(client.clone()).create_files(request).await?;
    // let file_id = response.id.clone();
    // println!("Created file: {:?}", &file_id);

    let response = Files::new(client.clone()).get_files().await?;
    println!(
        "All files: {:?}, Amount of files {}",
        response,
        response.data.len()
    );
    //
    let file_id = response.data[0].id.clone();
    let response = Files::new(client.clone())
        .get_file_by_id(file_id.clone())
        .await?;

    println!("Get file: {:?}", response);

    let response = Files::new(client.clone())
        .delete_file_by_id(file_id)
        .await?;
    match response.deleted {
        true => println!("File deleted: {:?}", response.id),
        false => println!("File is not deleted: {:?}", response.id),
    }

    Ok(())
}
