use async_gigachat::{
    client::Client,
    config::GigaChatConfig,
    embeddings::{Embeddings, EmbeddingsRequestBuilder},
    result::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::new();

    let client = Client::with_config(config);

    let request = EmbeddingsRequestBuilder::default()
        .input(vec![
            "Why shouldn't you leave the room?".to_owned(),
            "What is the mistake?".to_owned(),
        ])
        .build()?;

    let response = Embeddings::new(client).encode(request).await?;

    println!("response: {:?}", response);

    Ok(())
}
