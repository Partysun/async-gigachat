use async_gigachat::{
    chat::{Chat, ChatCompletionRequestBuilder, ChatMessageBuilder, Role},
    client::Client,
    config::GigaChatConfig,
    files::{Files, FilesRequest},
    result::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::new();

    let client = Client::with_config(config);

    let request = FilesRequest {
        file_path: "./examples/image_file_sample.png".to_string(),
        file_name: "image_file_sample.png".to_string(),
    };

    let response = Files::new(client.clone()).create_files(request).await?;
    let file_id = response.id.clone();
    println!("Created file: {:?}", &file_id);

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
    let choice = response.choices.get(0).unwrap();

    println!("{}: {}", question.role.unwrap(), question.content);
    println!(
        "{}: {}",
        choice.message.clone().role.unwrap(),
        choice.message.content
    );

    let response = Files::new(client.clone())
        .delete_file_by_id(file_id)
        .await?;
    match response.deleted {
        true => println!("File deleted: {:?}", response.id),
        false => println!("File is not deleted: {:?}", response.id),
    }
    Ok(())
}
