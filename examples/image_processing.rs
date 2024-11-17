use async_gigachat::{
    chat::{Chat, ChatCompletionRequestBuilder, ChatMessageBuilder, Role},
    client::Client,
    config::GigaChatConfig,
    files::Files,
    result::Result,
};
use std::{fs::File, io::Read};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::new();

    let client = Client::with_config(config);

    // Create image file from bytes
    let mut image_file = File::open("./examples/image_file_sample.png")?;
    let mut image_bytes = Vec::new();
    image_file.read_to_end(&mut image_bytes)?;
    let slice: &[u8] = &image_bytes;
    let response = Files::new(client.clone())
        .create_file(&slice, "image_file_sample.png".to_string(), "image/png")
        .await?;
    let file_id = response.id.clone();

    println!("Created file: {:?}", &file_id);

    // Ask GigaChat about this image
    let question = ChatMessageBuilder::default()
        .role(Role::User)
        .content("What is on this image?")
        .attachments(vec![file_id.clone()])
        .build()?;

    let request = ChatCompletionRequestBuilder::default()
        .messages(vec![question.clone()])
        .model("GigaChat-Pro")
        .build()?;

    let response = Chat::new(client.clone()).completion(request).await?;
    let choice = response.choices.first().unwrap();

    println!("{}: {}", question.role.unwrap(), question.content);
    println!(
        "{}: {}",
        choice.message.clone().role.unwrap(),
        choice.message.content
    );

    // Read all files
    let response = Files::new(client.clone()).get_files().await?;
    println!(
        "All files: {:?}, Amount of files {}",
        response,
        response.data.len()
    );

    let file_id = response.data[1].id.clone();
    let response = Files::new(client.clone())
        .get_file_by_id(file_id.clone())
        .await?;
    //
    println!("Get file: {:?}", response);

    // Delete this image
    let response = Files::new(client.clone())
        .delete_file_by_id(file_id)
        .await?;
    match response.deleted {
        true => println!("File deleted: {:?}", response.id),
        false => println!("File is not deleted: {:?}", response.id),
    }

    Ok(())
}
