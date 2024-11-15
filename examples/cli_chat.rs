use std::io::{stdin, stdout, Write};

use async_gigachat::{
    chat::{Chat, ChatCompletionRequestBuilder, ChatMessage, ChatMessageBuilder, Role},
    client::Client,
    config::GigaChatConfig,
    result::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::new();
    println!("{:?}", &config);

    let client: Client = Client::with_config(config);

    let mut messages: Vec<ChatMessage> = vec![];

    println!("help: type :q to end the chat\n");

    loop {
        let mut buffer = String::new();

        print!("user: ");
        stdout().flush()?;

        stdin().read_line(&mut buffer)?;

        let buffer = buffer.trim();

        if buffer.eq(":q") {
            break;
        }

        messages.push(
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(buffer)
                .build()?,
        );

        let request = ChatCompletionRequestBuilder::default()
            .messages(messages.clone())
            .model("GigaChat:latest")
            .build()?;

        let response = Chat::new(client.clone()).completion(request).await?;

        let message = &response.choices.get(0).unwrap().message.content;

        println!("assistant: {}", message);
        stdout().flush()?;

        messages.push(
            ChatMessageBuilder::default()
                .role(Role::Assistant)
                .content(message.clone())
                .build()?,
        );
    }

    Ok(())
}
