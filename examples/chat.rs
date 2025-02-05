use gigalib::{
    controllers::{
        chat::Chat,
        client::{ClientBuilder, GigaClient},
    },
    http::message::{MessageConfig, MessageConfigBuilder},
};

#[tokio::main]
async fn main() {
    // You don't have to create your MessageConfig, if no config is passed, default one is used
    let config: MessageConfig = MessageConfigBuilder::new()
        .set_max_tokens(999)
        .set_model("GigaChat-Max")
        .build();

    let client: GigaClient = ClientBuilder::new()
        .set_basic_token(&std::env::var("GIGACHAT_TOKEN").unwrap())
        .set_msg_cfg(config)
        .build();

    let mut chat: Chat = Chat::new(client);
    // or -> let mut chat: Chat = Chat::new_cached(client);

    chat.get_client_mut().reset_msg_config(None); // Set to default

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let resp = chat.send_message(input.into()).await.unwrap();
        println!("{}", resp.content);
    }
}
