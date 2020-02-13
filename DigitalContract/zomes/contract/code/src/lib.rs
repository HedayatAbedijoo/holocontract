/* Digital Contract

Using this app Alice and Bob can sign a digital contract which is reliable and nobody can tamper it.
Alice want to sign a contract with Bob:
Alice create a PublicContract entry with Hash of contract, and contractor address
Alice create a PrivateContract with the body and title of contract and link it to PublicContract
Bob will receive a direct message from Alice, with the full body of contract and address of PublicContract
Bob validate the received message by Hash of received contract and Hash of Public contract.
Bob can reject the contract and stop process. so Public contract is not beign signed by him.
Bob can accept the contract. So he creates a private contract and Sign the public contract with the same hash of his contract.

*/

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

    // Start of process. this is function should be called by starter of the process. Anyboy who want to create a new contract
    #[zome_fn("hc_public")]
    fn create_contract(
        title: String,
        contract_body: String,
        contractor_address: Address,
        timestamp: usize,
    ) -> ZomeApiResult<Vec<Address>> {
        privatecontract::create(title, contract_body, contractor_address, timestamp)
    }

    // second party of contract(contractor), should call this method if he wants to accept the contract which sent to him
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

    // General function to retreive any entry by address
    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    // List of all my private contracts. then we need to query relative public contract from DHT from UI. the address of Public Contract is inside PrivateContract
    #[zome_fn("hc_public")]
    pub fn my_contracts() -> ZomeApiResult<Vec<PrivateContract>> {
        privatecontract::get_my_contracts()
    }

    // It is validating if an Agent accepted a contract or not.
    #[zome_fn("hc_public")]
    pub fn is_public_contract_signed_by_me(
        public_contract_address: Address,
    ) -> ZomeApiResult<bool> {
        publiccontract::is_signed_by_me(public_contract_address)
    }

    // This is a receiver function on Peer-to-Peer communication.
    // TODO: this function should send an event to UI. then UI shows a windows to user with the info of contract with 2 buttons: 1)Accept 2)Reject
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
