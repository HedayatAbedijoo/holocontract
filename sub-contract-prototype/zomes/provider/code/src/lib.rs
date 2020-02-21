/*
This zome is responsible to create a contract and start the process of signature via direct message.
This dummy implementation is temprory and just for the prototype version and should be replaced by a real Digital-Contract, after proof of concept.
How it is working in Demo:
instead of creating a public-entry and finish the process for Subscriber to sign it.
it is just creating a Public-Entry with the signature of Subscriber.
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
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
use hdk::prelude::*;
use std::convert::TryInto;

use hdk::holochain_persistence_api::cas::content::Address;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;

use crate::content::Content;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use holochain_wasm_utils::api_serialization::{
    get_entry::{GetEntryOptions, GetEntryResult},
    get_links::GetLinksOptions,
};

use hdk::holochain_core_types::time::Timeout;

mod content;
use serde_json::value::Value;
mod message;
use crate::contract::Contract;
use crate::message::MessageBody;
mod contract;
/******************************** */
#[zome]
pub mod provider_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    fn get_my_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.clone())
    }
    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<String> {
        let pub_contr: Contract = hdk::utils::get_as_type(address.clone())?;
        Ok(pub_contr.signature)
    }
    /**************************** Course Entry Definition and Functions */
    #[entry_def]
    fn anchor_entry_definition() -> ValidatingEntryType {
        entry!(
            name: "content",
            description: "this is the definition of content, Subscribers can access to my Content if they signed a contract",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Content>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_feed(blog: String, comments: Vec<String>) -> ZomeApiResult<Address> {
        let new_item = Content::new(blog, comments);
        let new_etnry = new_item.get_entry();
        let addr = hdk::commit_entry(&new_etnry)?;
        Ok(addr)
    }

    #[zome_fn("hc_public")]
    fn get_my_blogs() -> ZomeApiResult<Vec<Content>> {
        content::get_my_contents()
    }

    #[zome_fn("hc_public")]
    fn get_subscription_blogs(
        contract_address: Address,
        signature: String,
    ) -> ZomeApiResult<String> {
        let pub_contr: Contract = hdk::utils::get_as_type(contract_address.clone())?;

        contract::is_subs_valid(
            contract_address.clone(),
            pub_contr.subscriber.clone(),
            signature.clone(),
        )?;

        // return Ok(vec![Content::new(
        //     format!("{}{}", contract_address, signature),
        //     [].to_vec(),
        // )]);
        let message = MessageBody::new(
            contract_address.clone(),
            signature.clone(),
            pub_contr.subscriber,
        );

        // Send direct message asking for private data for a subscription
        let result = hdk::send(
            pub_contr.provider,
            JsonString::from(message).to_string(),
            Timeout::default(),
        )?;

        return Ok(result);
        // return Ok(vec![Content::new(
        //     format!("{}{}", contract_address, signature),
        //     [].to_vec(),
        // )]);

        // //let success: Result<Vec<Content>, _> = JsonString::from_json(&result).try_into();
        // let success: Vec<Content> = serde_json::from_str(&result).unwrap();
        // Ok(success)
    }

    #[receive]
    pub fn receive(address: Address, msg: JsonString) -> String {
        let success: Result<MessageBody, _> = JsonString::from_json(&msg).try_into();
        match success {
            Err(err) => format!("Error: {}", err),
            Ok(result) => match message::validate_received_message(address, result.clone()) {
                Ok(_) => {
                    let pub_contr: Contract =
                        hdk::utils::get_as_type(result.contract_address.clone()).unwrap();
                    let content_result = content::get_subscription_contents(pub_contr);
                    return json!(&content_result).to_string();
                }
                Err(err) => format!("Error: there was an error validating the contract: {}", err),
            },
        }
    }
}
