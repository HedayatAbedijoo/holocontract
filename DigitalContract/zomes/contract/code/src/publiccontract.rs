use crate::contract::Contract;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::holochain_persistence_api::hash::*;
use hdk::prelude::*;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        signature::{Provenance, Signature},
        time::Timeout,
    },
    AGENT_ADDRESS,
};
use multihash::{encode, Hash};

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct PublicContract {
    pub contract_hash: String,
    pub starter_address: Address,    // agent who start the contract
    pub contractor_address: Address, // another party of contract
    pub timestamp: usize,
}

impl PublicContract {
    pub fn new(
        contract_hash: String,
        starter_contract: Address,
        contractor_address: Address,
        timestamp: usize,
    ) -> Self {
        PublicContract {
            starter_address: starter_contract,
            contractor_address: contractor_address,
            contract_hash: contract_hash,
            timestamp: timestamp,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("publiccontract".into(), self.into())
    }
}

pub fn publiccontract_entry_definition() -> ValidatingEntryType {
    entry!(
        name:"publiccontract",
        description:"This is the contract for my private chain source",
        sharing:Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|_validation_data: hdk::EntryValidationData<PublicContract>|{
            Ok(())
        }
    )
}

pub fn create(
    contract: Contract,
    starter_contract: Address,
    contractor_address: Address,
    timestamp: usize,
) -> ZomeApiResult<Address> {
    let hash_of_contract = contract.get_hash();

    let new_public_contract = PublicContract::new(
        hash_of_contract,
        starter_contract,
        contractor_address,
        timestamp,
    );
    let public_contract_entry = new_public_contract.entry();
    let pub_cont_addr = hdk::commit_entry(&public_contract_entry)?;

    Ok(pub_cont_addr)
}

pub fn is_signed_by_me(public_contract_address: Address) -> ZomeApiResult<bool> {
    let signature = hdk::sign(public_contract_address.clone())?;
    let provenance = Provenance::new(AGENT_ADDRESS.clone(), Signature::from(signature));
    let validate_signature =
        hdk::verify_signature(provenance.clone(), public_contract_address.clone())?;

    if !validate_signature {
        return Err(ZomeApiError::from(String::from(
            "Error: You did not sign this contract",
        )));
    } else {
        Ok(true)
    }
}
