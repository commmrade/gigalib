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

    let file_info = client.upload_file("path/to/file.png".into()).await.unwrap();

    let mut msg = Message::from_str("What do you see on the picture?");
    msg.add_attachment(&file_info.id);

    let response: Message = client.send_message(msg).await.unwrap();
    println!("{}", response.content);
}
