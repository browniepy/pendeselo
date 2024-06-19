pub use ::serde::{Deserialize, Serialize};
pub use ::std::collections::HashMap;

mod client;
pub use client::Client;

pub mod contents;
pub use contents::Message;

mod chat;
pub use chat::Chat;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Assistant,
    User,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Model {
    #[serde(rename = "google/gemma-7b-it:free")]
    Gemma,
}

pub mod req_res;
