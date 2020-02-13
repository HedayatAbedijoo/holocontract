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
use std::str;

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

    pub fn validate_on_modify(&self, new_entry: &PublicContract) -> Result<(), String> {
        if self.contract_hash != new_entry.contract_hash
            || self.contractor_address != new_entry.contractor_address
            || self.starter_address != new_entry.starter_address
            || self.timestamp != new_entry.timestamp
        {
            return Err("Error: You can not change any element of contract".to_string());
        }
        Ok(())
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
        validation:|validation_data: hdk::EntryValidationData<PublicContract>|{
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    if !validation_data.sources().contains(&entry.starter_address) {
                        return Err(String::from("Error: You just can create a contract for yourself"));
                    }
                    if validation_data.sources().contains(&entry.contractor_address) {
                        return Err(String::from("Error: Contracter of contract is signed it now. it is not possible."));
                    }

                    if entry.contract_hash.is_empty() == true{
                        return Err(String::from("Error: Hash of contract can not be empty"));
                    }
                    if &entry.starter_address == &entry.contractor_address{
                        return Err(String::from("Error: Starter contract can not be equal as contractor"));
                    }
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    // Only Starter and Contractor of contract can modify Public Contract. And only just for Signature.
                    if !validation_data.sources().contains(&old_entry.starter_address) ||
                    !validation_data.sources().contains(&old_entry.contractor_address){
                        return Err(String::from("Error: You tried to modify the contract which is not yours"));
                    }
                    old_entry.validate_on_modify(&new_entry)
                },
                EntryValidationData::Delete {.. } => {
                    Err(String::from("Error: You can not delete a public contract"))
                }
            }
        },
        links: [
            to!( // to query all my public contracts
                "%agent_id",
                link_type: "agent->publiccontracts",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}
// this functino is a starter of process. Agent who wants to start a contract will call this.
pub fn create(
    contract: Contract,
    starter_contract: Address,
    contractor_address: Address,
    timestamp: usize,
) -> ZomeApiResult<Address> {
    let hash_of_contract = contract.get_hash();

    // create a Public contract with a Hash of all contract body and title
    let new_public_contract = PublicContract::new(
        hash_of_contract,
        starter_contract,
        contractor_address,
        timestamp,
    );
    let public_contract_entry = new_public_contract.entry();
    let pub_cont_addr = hdk::commit_entry(&public_contract_entry)?;
    // create a link between my agent to public contract for query and search
    hdk::link_entries(&AGENT_ADDRESS, &pub_cont_addr, "agent->publiccontracts", "")?;

    Ok(pub_cont_addr)
}

// We need a validation to see a claimed public contract is already signed by me. which mean did I accept this contract
// TODO: later: we need full cycle validation. we need to find private contract connected to public contract.Err
// we need to validate the Hash of private contract with a Hash of connected public contract
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
