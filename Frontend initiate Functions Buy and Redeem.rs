// initiate transactions for the Redeem and Buy functions from the front end of your dapp using Rust and Yew:

//To initiate a transaction for the Redeem function, which requires a token spend greater than 1 and passes parameters name and address to a wasm module on Ethereum:

use web3::{
    contracts::{Contract, Options}, // import the `Contract` and `Options` structs from the `contracts` module of the `web3` crate
    types::{Address, H256, U256}, // import the `Address`, `H256`, and `U256` types from the `types` module of the `web3` crate
    Web3, // import the `Web3` struct from the `web3` crate
};

#[wasm_bindgen] // decorator to generate the necessary JavaScript binding code for this function
pub async fn redeem(web3: Web3<Http>, contract_address: H256, name: String, address: String) {
    // `web3` is an instance of the `Web3` struct that is connected to your Ethereum node
    // `contract_address` is the address of your smart contract on the Ethereum blockchain
    // `name` and `address` are the parameters that will be passed to the `redeem` function on the smart contract

    let contract = Contract::new(web3.eth(), contract_address, Options::default()).unwrap();
    // create a new instance of the `Contract` struct with the given `web3` instance, `contract_address`, and default options
    // the `Contract` struct allows you to call functions on the smart contract and encode




let name_bytes = name.as_bytes().to_vec();
let address_bytes = address.as_bytes().to_vec();
// convert the `name` and `address` strings to byte vectors

let data = contract.functions().redeem(name_bytes, address_bytes).encode_input();
// call the `redeem` function on the `Contract` instance and encode the arguments as data that can be passed in a transaction

let from = web3.eth().accounts().await.unwrap()[0];
// get the first account from the list of accounts returned by the `accounts` method of the `Web3` instance
// this account will be used as the `from` address in the transaction

let value = U256::from(2);
// set the value of the transaction to 2 tokens

let tx = web3.eth().transact_contract(from, contract_address, value, data, Options::default());
// create a transaction object with the `from` address, `contract_address`, `value`, and `data`

let receipt = web3.eth().send_transaction(tx).await.unwrap();
// send the transaction and wait for the receipt
// the receipt contains the details of the transaction, including the transaction hash, block number, and gas used
To initiate a transaction for the Buy function, which requires 1 eth and sends no parameters:

use web3::{
    contracts::{Contract, Options}, // import the `Contract` and `Options` structs from the `contracts` module of the `web3` crate
    types::{Address, H256, U256}, // import the `Address`, `H256`, and `U256` types from the `types` module of the `web3` crate
    Web3, // import the `Web3` struct from the `web3` crate
};

#[wasm_bindgen] // decorator to generate the necessary JavaScript binding code for this function
pub async fn buy(web3: Web3<Http>, contract_address: H256) {
    // `web3` is an instance of the `Web3` struct that is connected to your Ethereum node
    // `contract_address` is the address of


let contract = Contract::new(web3.eth(), contract_address, Options::default()).unwrap();
// create a new instance of the `Contract` struct with the given `web3` instance, `contract_address`, and default options
// the `Contract` struct allows you to call functions on the smart contract and encode the arguments as data that can be passed in a transaction

let data = contract.functions().buy().encode_input();
// call the `buy` function on the `Contract` instance and encode the arguments as data that can be passed in a transaction

let from = web3.eth().accounts().await.unwrap()[0];
// get the first account from the list of accounts returned by the `accounts` method of the `Web3` instance
// this account will be used as the `from` address in the transaction

let value = U256::from(1_000_000_000_000_000_000u64);
// set the value of the transaction to 1 ether

let tx = web3.eth().transact_contract(from, contract_address, value, data, Options::default());
// create a transaction object with the `from` address, `contract_address`, `value`, and `data`

let receipt = web3.eth().send_transaction(tx).await.unwrap();
// send the transaction and wait for the receipt
// the receipt contains the details of the transaction, including the transaction hash, block number, and gas used


