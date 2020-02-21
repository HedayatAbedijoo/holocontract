use crate::contract;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        signature::{Provenance, Signature},
        time::Timeout,
    },
    holochain_persistence_api::cas::content::Address,
    AGENT_ADDRESS,
};
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageBody {
    pub contract_address: Address,
    pub signature: String,
    pub subscriber_address: Address,
}

impl MessageBody {
    pub fn new(contract_address: Address, signature: String, subscriber_address: Address) -> Self {
        MessageBody {
            contract_address,
            signature,
            subscriber_address,
        }
    }
}

pub fn validate_received_message(sender_address: Address, msg: MessageBody) -> ZomeApiResult<()> {
    contract::is_subs_valid(msg.contract_address, sender_address, msg.signature)
}
