use crate::contract::Contract;
use crate::publiccontract::PublicContract;
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
    pub contract: Contract,
    pub signature: String,
    pub public_contract_address: Address,
}

// Direct communication between parties. When Alice create a new Contract with Bob. a direct message should be sent to Bob
pub fn send_contract(
    receiver_addrs: Address,
    pub_addr_contract: Address,
    contract: Contract,
) -> Result<(), String> {
    let signature = hdk::sign(pub_addr_contract.clone())?;
    let message = MessageBody {
        contract: contract,
        signature: signature,
        public_contract_address: pub_addr_contract,
    };

    let result = hdk::send(
        receiver_addrs,
        JsonString::from(message).to_string(),
        Timeout::default(),
    )?;

    hdk::debug("_this_is_from_inside")?;

    if result.contains("Error") {
        Err(result)
    } else {
        Ok(())
    }
}

// When a receiver, receives a new direct message, which is a new contract. we need to evaluate it.
pub fn validate_received_message(sender_address: Address, msg: MessageBody) -> Result<(), String> {
    let pub_contr: PublicContract = hdk::utils::get_as_type(msg.public_contract_address.clone())?;
    // Validate Sender with Public Contract
    if pub_contr.starter_address.to_string() != sender_address.to_string() {
        return Err("Error: Sender address is not equal to public contract starter".to_string());
    }
    // Validate Sender Signature on Public Contract. Sender should be an agent who signed the Public Contract
    let provenance = Provenance::new(sender_address.clone(), Signature::from(msg.signature));
    let validate_signature =
        hdk::verify_signature(provenance.clone(), msg.public_contract_address.clone())?;
    if !validate_signature {
        return Err(
            "Error: Message signature is not equal to public contract signature".to_string(),
        );
    }
    // Validate Hash of Contract. The contract that I received should be equal to the Public Contract
    if msg.contract.get_hash() != pub_contr.contract_hash {
        return Err(
            "Error: The received contract is not equal as Public contract. It is faked".to_string(),
        );
    }
    Ok(())
}
