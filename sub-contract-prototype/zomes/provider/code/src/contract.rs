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
pub struct Contract {
    pub contract_type: String,
    pub subscriber: Address, // temprory instead of signature in header
    pub signature: String,
    pub provider: Address, // it should be removed later
}

pub fn is_subs_valid(
    contract_address: Address,
    subscriber_address: Address,
    signature: String,
) -> ZomeApiResult<()> {
    let is_valid_json: JsonString = hdk::call(
        hdk::THIS_INSTANCE,
        "contract",
        Address::from(hdk::PUBLIC_TOKEN.to_string()),
        "validate_subscription",
        json!({ "contract_address": contract_address, "subscriber_address":subscriber_address, "signature":signature }).into(),
    )?;

    let is_valid: Result<ZomeApiResult<bool>, _> = serde_json::from_str(&is_valid_json.to_string());
    match is_valid {
        Ok(Ok(true)) => Ok(()),
        _ => Err(ZomeApiError::from(String::from(
            "The subscribtion contract is not valid",
        ))),
    }
}
