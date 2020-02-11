use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::hash::*;
use multihash::{encode, Hash};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Contract {
    pub title: String,
    pub body: String,
}

impl Contract {
    pub fn new(title: String, body: String) -> Self {
        Contract {
            title: title,
            body: body,
        }
    }

    pub fn get_hash(&self) -> String {
        return HashString::encode_from_str(&self.body, Hash::SHA2256).to_string();
    }
}
