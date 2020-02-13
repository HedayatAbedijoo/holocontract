
# Digital Contract on Holochain

## Using this app Alice and Bob can sign a digital contract which is reliable and nobody can tamper it.


### Example: Alice wants to sign a contract with Bob:
- Alice create a Public-Contract entry on DHT with the Hash of contract plus contractor address.
- Then Alice create a Private-Contract on her local-chain with the body and title of contract with a link to the Public-Contract address.
- Then Alice send a direct message to Bob with the full content of the contract and the address of Public-Contract on DHT.
- Bob will receive a direct message from Alice, and validate the received message by comparing Hash of Public-Contract with Hash of contract he received directly from Alice.
- Then Bob can reject the contract and stop the process. Which means the Public-Contract on DHT is not being signed by him.
- Or Bob can accept the contract. So he creates a new Private-Contract on his local-chain and Sign the public contract on DHT.

```rust

pub struct Contract {
    pub title: String,
    pub body: String,
}

// This entry save on DHT
pub struct PublicContract {
    pub contract_hash: String,       // Hash of the whole contract. so nobody can have different version of contract
    pub starter_address: Address,    // agent who start the contract. Alice public key
    pub contractor_address: Address, // another party of contract. Bob public key
    pub timestamp: usize,
}

// This entry just save on Local-chain of each agent. not on the DHT
pub struct PrivateContract {
    pub contract: Contract,                // full version of contract that each party save on his local-chain
    pub starter_address: Address,         // agent who start the contract. Alice public key
    pub contractor_address: Address,      // another party of contract. Bob public key
    pub public_contract_address: Address, // the address of Public Contract on DHT
    pub timestamp: usize,
}
```

### Validation rules of Holochain application stops any party to change Public-Contract so it will be tamper proof Digital Contract.
