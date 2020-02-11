// use hdk::holochain_json_api::{error::JsonError, json::JsonString};
// use hdk::holochain_persistence_api::cas::content::Address;
// use hdk::prelude::*;
// use hdk::{
//     entry_definition::ValidatingEntryType,
//     error::{ZomeApiError, ZomeApiResult},
//     AGENT_ADDRESS,
// };
// use std::convert::TryFrom;
use hdk::prelude::*;
use holochain_wasm_utils::api_serialization::query::QueryArgsNames;

use crate::contract::Contract;
use crate::message;
use crate::publiccontract;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::{entry_definition::ValidatingEntryType, AGENT_ADDRESS};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
};
use hdk_proc_macros::zome;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct PrivateContract {
    pub contract: Contract,
    pub starter_address: Address,         // agent who start the contract
    pub contractor_address: Address,      // another party of contract
    pub public_contract_address: Address, // the address of Hash on DHT
    pub timestamp: usize,
}

impl PrivateContract {
    pub fn new(
        contract: Contract,
        starter_address: Address,
        contractor_address: Address,
        public_contract_address: Address,
        timestamp: usize,
    ) -> Self {
        PrivateContract {
            starter_address: starter_address,
            contractor_address: contractor_address,
            public_contract_address: public_contract_address,
            timestamp: timestamp,
            contract: contract,
        }
    }

    pub fn from_entry(entry: &Entry) -> Option<PrivateContract> {
        match entry {
            Entry::App(entry_type, privatecontract_entry) => {
                if entry_type.to_string() != "privatecontract" {
                    return None;
                }

                match PrivateContract::try_from(privatecontract_entry) {
                    Ok(t) => Some(t),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("privatecontract".into(), self.into())
    }
}

pub fn privatecontract_entry_definition() -> ValidatingEntryType {
    entry!(
        name:"privatecontract",
        description:"This is the contract for my private chain source",
        sharing:Sharing::Private,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|_validation_data: hdk::EntryValidationData<PrivateContract>|{
            Ok(())
        }
    )
}

pub fn create(
    title: String,
    contract_body: String,
    contractor_address: Address,
    timestamp: usize,
) -> ZomeApiResult<Address> {
    let contract = Contract::new(title, contract_body);

    let public_contract_address = publiccontract::create(
        contract.clone(),
        AGENT_ADDRESS.to_string().into(),
        contractor_address.clone(),
        timestamp,
    )?;

    let private_contect = PrivateContract::new(
        contract.clone(),
        contractor_address.clone(),
        public_contract_address.clone(),
        AGENT_ADDRESS.to_string().into(),
        timestamp,
    );

    let private_contract_entry = private_contect.entry();
    let _private_entry_address = hdk::commit_entry(&private_contract_entry)?;

    message::send_contract(
        contractor_address,
        public_contract_address.clone(),
        contract.clone(),
    )?;
    Ok(public_contract_address)
}

// Get all private contracts of the current Agent
pub fn get_my_contracts() -> ZomeApiResult<Vec<PrivateContract>> {
    let chain_entries = get_chain_entries("privatecontract".into())?;

    Ok(chain_entries
        .into_iter()
        .filter_map(|entry| PrivateContract::from_entry(&entry.1))
        .collect())
}
fn get_chain_entries(entry_type: String) -> ZomeApiResult<Vec<(Address, Entry)>> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        headers: false,
        entries: true,
    };
    let result = hdk::query_result(QueryArgsNames::from(entry_type), options)?;
    match result {
        QueryResult::Entries(entries) => Ok(entries),
        _ => Err(ZomeApiError::from(String::from(
            "Error when getting own transactions",
        ))),
    }
}
