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
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::prelude::*;

use hdk::holochain_persistence_api::cas::content::Address;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;

//use std::convert::TryInto;

/******************************** */
#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Contract {
    pub contract_type: String,
    pub subscriber: Address, // temprory instead of signature in header
    pub signature: String,
    pub provider: Address, // it should be removed later
}

impl Contract {
    pub fn new(contract_type: String, subscriber: Address) -> Self {
        Contract {
            contract_type,
            subscriber,
            signature: String::default(),
            provider: AGENT_ADDRESS.to_string().into(),
        }
    }

    pub fn get_entry(&self) -> Entry {
        Entry::App("contract".into(), self.into())
    }
}

#[zome]
pub mod contract_zome {

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

    /**************************** Course Entry Definition and Functions */
    #[entry_def]
    fn anchor_entry_definition() -> ValidatingEntryType {
        entry!(
            name: "contract",
            description: "this is the definition of course",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Contract>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_subscribe_contract(
        subscriber: Address,
        contract_type: String,
    ) -> ZomeApiResult<Address> {
        let contract = Contract::new(contract_type, subscriber);
        let enty_contract = contract.get_entry();
        let addr = hdk::commit_entry(&enty_contract)?;
        Ok(addr)
    }

    #[zome_fn("hc_public")]
    fn sign_contract_by_subscriber(
        contract_address: Address,
        signature: String,
    ) -> ZomeApiResult<Address> {
        let mut contract: Contract = hdk::utils::get_as_type(contract_address.clone())?;
        contract.signature = signature;
        let adrr = hdk::update_entry(contract.get_entry(), &contract_address)?;
        Ok(adrr)
    }

    #[zome_fn("hc_public")]
    fn validate_subscription(
        contract_address: Address,
        subscriber_address: Address,
        signature: String,
    ) -> ZomeApiResult<bool> {
        let contract: Contract = hdk::utils::get_as_type(contract_address.clone())?;
        if contract.signature.is_empty() {
            return Ok(false);
        }
        if contract.subscriber != subscriber_address || contract.signature != signature {
            return Ok(false);
        }
        return Ok(true);
    }

    #[zome_fn("hc_public")]
    pub fn get_my_signature(entry_address: Address) -> ZomeApiResult<String> {
        let signature = hdk::sign(entry_address.clone())?;
        return Ok(signature);
    }
}
