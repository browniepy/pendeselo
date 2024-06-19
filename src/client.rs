use crate::{
    req_res::{Request, Response},
    Chat, HashMap, Message, Model,
};

pub struct Client {
    pub http: ::reqwest::Client,
    pub api_key: String,
    pub chats: HashMap<i64, Chat>,
    pub model: Model,
}

impl Client {
    pub fn new(api_key: impl Into<String>, model: Model) -> Self {
        Self {
            http: ::reqwest::Client::new(),
            api_key: api_key.into(),
            chats: HashMap::new(),
            model,
        }
    }

    pub async fn send(&self, message: Message) -> Result<Response, ::reqwest::Error> {
        Ok(self
            .dispatch(Request::new(message, self.model.clone()))
            .await
            .unwrap())
    }

    async fn dispatch(&self, req: Request) -> Result<Response, ::reqwest::Error> {
        let base = "https://openrouter.ai/api/v1/chat/completions";
        let response: Response = self
            .http
            .post(base)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
