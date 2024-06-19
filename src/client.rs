use crate::{chat::Chat, HashMap};

pub struct Client {
    pub http: ::reqwest::Client,
    pub api_key: String,
    pub chats: HashMap<i64, Chat>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            http: ::reqwest::Client::new(),
            api_key: ::std::env::var("or_key").unwrap(),
            chats: HashMap::new(),
        }
    }
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: ::reqwest::Client::new(),
            api_key: api_key.into(),
            chats: HashMap::new(),
        }
    }
}
