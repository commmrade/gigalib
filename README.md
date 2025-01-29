
Usage example:
```rust
use gigalib::{clients::{chat::Chat, client::ClientBuilder}, http::message::MessageConfigBuilder};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cfg = MessageConfigBuilder::new().set_model("GigaChat").set_max_tokens(512).build();

    let tok = std::env::var("GIGACHAT_KEY").unwrap();
    let client = ClientBuilder::new()
    .set_basic_token(&tok)
    .set_msg_cfg(cfg)
    .build();

    let mut chat = Chat::new_cached(client);

    let models = chat.get_client_mut().get_models().await.unwrap();
    let mut cfg = chat.get_client().get_current_config();
    cfg.model = models.last().unwrap().id.clone();

    chat.get_client_mut().reset_msg_config(Some(cfg));

    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();

        let resp = chat.send_message(str.into()).await.unwrap();
        println!("{}", resp.content);

        if resp.content == "exit" {
            break
        }
    }   
}
```