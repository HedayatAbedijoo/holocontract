
# Digital Contracts on Holochain

Using this app Alice and Bob can sign a digital contract which is reliable and nobody can tamper it.
Alice want to sign a contract with Bob:
Alice create a PublicContract entry with Hash of contract, and contractor address
Alice create a PrivateContract with the body and title of contract and link it to PublicContract
Bob will receive a direct message from Alice, with the full body of contract and address of PublicContract
Bob validate the received message by Hash of received contract and Hash of Public contract.
Bob can reject the contract and stop process. so Public contract is not beign signed by him.
Bob can accept the contract. So he creates a private contract and Sign the public contract with the same hash of his contract.
