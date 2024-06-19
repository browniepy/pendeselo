use crate::{
    contents::{Content, Message, Usage},
    Deserialize, Model, Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub choices: Vec<Content>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub model: Model,
    pub messages: Vec<Message>,
}

impl Request {
    pub fn new(message: Message, model: Model) -> Self {
        let messages = vec![message];
        Self { model, messages }
    }
}

impl Response {
    pub fn content(&self) -> &String {
        let cont = self.choices.first().unwrap();
        &cont.message.content
    }
}
