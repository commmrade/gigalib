use gigalib::{
    controllers::client::{ClientBuilder, GigaClient},
    http::message::{Message, MessageConfig, MessageConfigBuilder},
};

#[tokio::main]
async fn main() {
    // You don't have to create your MessageConfig, if no config is passed, default one is used
    let config: MessageConfig = MessageConfigBuilder::new()
        .set_max_tokens(999)
        .set_model("GigaChat-Pro")
        .build();

    let mut client: GigaClient = ClientBuilder::new()
        .set_basic_token(&std::env::var("GIGACHAT_TOKEN").unwrap())
        .set_msg_cfg(config)
        .build();

    // There are a lot of different ways to pass a message into send_message()
    let response: Message = client.send_message("hello!".into()).await.unwrap();
    let response = client
        .send_message(String::from("Hello!").into())
        .await
        .unwrap();
    let response = client
        .send_message(Message::from_str("hello"))
        .await
        .unwrap();
}
