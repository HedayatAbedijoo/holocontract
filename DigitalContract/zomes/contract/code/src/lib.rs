/***************** Required Library */
#![feature(vec_remove_item)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
use hdk::holochain_json_api::json::JsonString;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;
use holochain_wasm_utils::api_serialization::query::QueryArgsNames;
use std::convert::TryInto;
extern crate multihash;
/******************************** */

pub mod contract;
mod message;
mod privatecontract;
mod publiccontract;
use crate::message::MessageBody;
use crate::privatecontract::PrivateContract;
#[zome]
mod contract_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validate_data: EntryValidationData<AgentId>) {
        Ok(())
    }
    #[entry_def]
    fn public_contract_entry() -> ValidatingEntryType {
        publiccontract::publiccontract_entry_definition()
    }

    #[entry_def]
    fn private_contract_entry() -> ValidatingEntryType {
        privatecontract::privatecontract_entry_definition()
    }

    #[zome_fn("hc_public")]
    fn create_contract(
        title: String,
        contract_body: String,
        contractor_address: Address,
        timestamp: usize,
    ) -> ZomeApiResult<Vec<Address>> {
        privatecontract::create(title, contract_body, contractor_address, timestamp)
    }

    #[zome_fn("hc_public")]
    pub fn confirm_contract(
        public_contract_address: Address,
        title: String,
        body: String,
        timestamp: usize,
    ) -> ZomeApiResult<Vec<Address>> {
        let contr = contract::Contract::new(title, body);
        privatecontract::confirm(public_contract_address, contr, timestamp)
    }

    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    pub fn my_contracts() -> ZomeApiResult<Vec<PrivateContract>> {
        privatecontract::get_my_contracts()
    }

    #[zome_fn("hc_public")]
    pub fn is_public_contract_signed_by_me(
        public_contract_address: Address,
    ) -> ZomeApiResult<bool> {
        publiccontract::is_signed_by_me(public_contract_address)
    }

    #[receive]
    pub fn receive(address: Address, msg: JsonString) -> String {
        let success: Result<MessageBody, _> = JsonString::from_json(&msg).try_into();
        match success {
            Err(err) => format!("Error: {}", err),
            Ok(result) => match message::validate_received_message(address, result) {
                Ok(_) => "OK: Message received sucessfully".to_string(),
                Err(err) => format!("Error: there was an error validating the contract: {}", err),
            },
        }
    }
}
