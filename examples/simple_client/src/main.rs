use router::{
    Client,
    Message
};

#[tokio::main]
async fn main() {
    let client = Client::new("API key", router::Model::Gemma);
    let msg_result = client.send(
        Message::from_user("Hello, world!")
    ).await;

    if let Err(why) = msg_result {
        panic!("There's an error: {}", why);
    }
}