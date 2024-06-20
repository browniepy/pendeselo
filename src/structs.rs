use crate::{Deserialize, Role, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub message: Message,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl Message {
    pub fn from_user(content: impl Into<String>) -> Self {
        let role = Role::User;
        let content = content.into();
        Self { role, content }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub choices: Vec<Content>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub model: crate::Model,
    pub messages: Vec<Message>,
}

impl Request {
    pub fn new(message: Message, model: crate::Model) -> Self {
        let messages = vec![message];
        Self { model, messages }
    }
}

impl Response {
    pub fn message(&self) -> Message {
        let cont = self.choices.first().unwrap();
        cont.message
    }
}
