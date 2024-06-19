pub use ::serde::{Deserialize, Serialize};
pub use ::std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Assistant,
    User,
}

#[derive(Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "google/gemma-7b-it:free")]
    Gemma,
}

pub mod chat;
pub mod client;
pub mod contents;
pub mod req_res;
