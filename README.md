
Usage example:
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut cfg = MessageConfigBuilder::new().set_model("GigaChat").set_max_tokens(512).build();

    let tok = std::env::var("GIGACHAT_KEY").unwrap();
    let mut client = ClientBuilder::new()
    .set_basic_token(&tok)
    .set_msg_cfg(cfg)
    .build();

    let mut chat = Chat::new(client);

    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();

        let resp = chat.send_message(str.into()).await.unwrap();
        println!("{}", resp.content);

        if resp.content == "exit" {
            break
        }
    }   


    let mut cfg = MessageConfigBuilder::new().set_max_tokens(30).build();
    chat.get_client_mut().reset_msg_config(cfg.into());
}
```