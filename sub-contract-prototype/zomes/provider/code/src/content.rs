use crate::contract::Contract;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::*;
use hdk::{entry_definition::ValidatingEntryType, AGENT_ADDRESS};
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_core_types::{
        signature::{Provenance, Signature},
        time::Timeout,
    },
    holochain_persistence_api::cas::content::Address,
};
use hdk_proc_macros::zome;
use holochain_wasm_utils::api_serialization::query::QueryArgsNames;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, holochain_json_derive::DefaultJson, Clone)]
pub struct Content {
    pub blog: String,
    pub comments: Vec<String>,
}

impl Content {
    pub fn new(blog: String, comments: Vec<String>) -> Self {
        Content { blog, comments }
    }

    pub fn get_entry(&self) -> Entry {
        Entry::App("content".into(), self.into())
    }

    pub fn from_entry(entry: &Entry) -> Option<Content> {
        match entry {
            Entry::App(entry_type, contract_entry) => {
                if entry_type.to_string() != "content" {
                    return None;
                }

                match Content::try_from(contract_entry) {
                    Ok(t) => Some(t),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

// Get all private contracts of the current Agent
pub fn get_my_contents() -> ZomeApiResult<Vec<Content>> {
    let chain_entries = get_chain_entries("content".into())?;

    Ok(chain_entries
        .into_iter()
        .filter_map(|entry| Content::from_entry(&entry.1))
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

pub fn get_subscription_contents(contract: Contract) -> ZomeApiResult<Vec<Content>> {
    match contract.contract_type.as_ref() {
        "gold-membership" => {
            let chain_entries = get_chain_entries("content".into())?;

            Ok(chain_entries
                .into_iter()
                .filter_map(|entry| {
                    return Content::from_entry(&entry.1);
                })
                .collect())
        }
        "silver-membership" => {
            let chain_entries = get_chain_entries("content".into())?;

            Ok(chain_entries
                .into_iter()
                .filter_map(|entry| {
                    let content: Content = Content::from_entry(&entry.1).unwrap();
                    return Some(Content::new(content.blog, Vec::default()));
                })
                .collect())
        }
        _ => Err(ZomeApiError::from("Your permission is denied!".to_string())),
    }
}
