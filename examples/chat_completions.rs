use async_gigachat::{
    chat::{Chat, ChatCompletionRequestBuilder, ChatMessageBuilder, Role},
    client::Client,
    config::GigaChatConfig,
    result::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::new();

    let client: Client = Client::with_config(config);

    let question = ChatMessageBuilder::default()
        .role(Role::User)
        .content("Hey, how's it going?")
        .build()?;

    let request = ChatCompletionRequestBuilder::default()
        .messages(vec![question.clone()])
        .model("GigaChat:latest")
        .build()?;

    let response = Chat::new(client).completion(request).await?;
    let choice = response.choices.get(0).unwrap();

    println!("{}: {}", question.role.unwrap(), question.content);
    println!(
        "{}: {}",
        choice.message.clone().role.unwrap(),
        choice.message.content
    );

    Ok(())
}
