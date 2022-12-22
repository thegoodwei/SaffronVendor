
/*
This front-end web application is a vendor website for selling and redeeming saffron. It allows users to purchase saffron using Ethereum, and it also allows users to redeem saffron they have already purchased by providing their name and delivery address.

The website is built using the Rust programming language and the Yew framework, which allows for the creation of reactive web applications. It also makes use of the Web3 library, which allows for the interaction with Ethereum smart contracts.

The Model struct represents the state of the component. It includes several state variables such as show_lightbox, lightbox_type, quantity, name, address, and redeem_quantity. These variables are used to track the state of the lightbox and the input values entered by the user when purchasing or redeeming saffron.

The LightboxType enum defines the possible types of lightboxes that can be displayed on the website. It includes the values Buy and Redeem, which correspond to the lightboxes for purchasing and redeeming saffron, respectively.

The Msg enum represents the possible messages that the component can receive. It includes messages for toggling the lightbox, setting the state variables, and confirming a purchase or redemption.

The Component trait is implemented for the Model struct. This allows the Model to behave as a component in the Yew framework, with functions such as create, update, and view.

The view_lightbox function is responsible for rendering the HTML elements of the lightbox. It includes a form for the user to enter their input values, and it also includes buttons for confirming the purchase or redemption.

The view function is responsible for rendering the HTML elements of the component. It includes the view_lightbox function to render the lightbox, as well as the main content of the website which consists of an image and buttons to trigger the lightbox to display.

The update function is called whenever the component's state needs to be updated. It receives a message and matches on the type of message to determine how to update the state. It includes logic for toggling the lightbox, setting the state variables, and confirming a purchase or redemption.

The main function is the entry point of the web application. It creates a new Model and creates a ComponentLink to the model. It then creates a new App with the Model and ComponentLink as arguments, and mounts the App to the DOM.

The encode_function_call function is a helper function that encodes the function call data for a given function of a smart contract. It takes in the function name and arguments, and it returns the encoded function call data as a hex string.

The buy_saffron function is responsible for initiating a purchase of saffron. It first gets the Ethereum address of the user who is currently logged in to their wallet. It then creates a contract object using the ABI and contract address that were defined earlier. This contract object allows us to interact with the functions of the smart contract.

Next, the function calls the contract's buy function by sending a transaction to the Ethereum network. The from parameter specifies the sender address, and the value parameter specifies the amount of Ether to send. The function also logs a message to the browser console to confirm that the transaction was sent.

The redeem_saffron function is responsible for initiating a redemption of saffron. It takes in two string arguments: name and address, which represent the name and delivery address of the user who is redeeming saffron.

The function first gets the Ethereum address of the user who is currently logged in to their wallet. It then creates a contract object using the ABI and contract address that were defined earlier. This contract object allows us to interact with the functions of the smart contract.

Next, the function converts the name and address strings to byte arrays using the web3.utils.fromAscii method. These byte arrays are then passed as arguments to the contract's redeem function when it is called by sending a transaction to the Ethereum network. The from parameter specifies the sender address, and the name and address parameters are the inputs to the function. The function also logs a message to the browser console to confirm that the transaction was sent.

To implement these functions with a Rust smart contract, you will need to do the following:

Install the Rust programming language and the necessary dependencies for Yew and Web3.
Create a new Rust project using cargo new.
Add Yew and Web3 as dependencies in the Cargo.toml file.
Create a new file for the front-end component, such as src/lib.rs.
Import the necessary modules for Yew and Web3 at the top of the file.
Define the Model struct and the LightboxType and Msg enums as shown in the code above.
Implement the Component trait for the Model struct.
Define the view_lightbox and view functions to render the
user interface elements and the lightbox.

9. In the update function, handle the different messages that the component can receive and update the state variables accordingly.

In the view function, use the state variables to render the different elements of the user interface, including the buttons for initiating purchases and redemptions.
To interact with the smart contract, you will need to define the ABI and contract address as constants, just like in the JavaScript code.
Define the buy and redeem_saffron functions as shown in the code above.
Initialize the Web3 provider using the window.ethereum object, or a different provider if necessary.
Load the contract using the ABI and contract address, and call the buy or redeem function by sending a transaction to the Ethereum network.
In the main function, create an instance of the component and mount it to the DOM.
It is also recommended to add error handling and input validation to the functions to ensure a smooth user experience. For example, you can use the try_parse method to validate the input values before parsing them as integers, and you can use the catch method to handle any errors that may occur during the interaction with the smart contract.

10.
To illustrate how the Rust smart contract can interact with this front-end web app, let's consider the following example:

First, you will need to deploy your Rust smart contract to the Ethereum network. You can do this using a tool such as Remix, Truffle, or the Rust Ethereum Development Kit (REDK).
Once your contract is deployed, you will need to copy the contract address and ABI. These will be used to interact with the contract from the front-end web app.
In the Rust front-end web app, define the ABI and contract address as constants, like this:
Copy code
const abi = [
    // The ABI of your smart contract goes here
];

const contract_address = "0x..."; // Replace with the actual contract address
Initialize the Web3 provider using the window.ethereum object or a different provider if necessary.
Copy code
const web3 = new Web3(window.ethereum);
Load the contract using the ABI and contract address, and create a contract object that you can use to interact with the contract's functions.
Copy code
const contract = new web3.eth.Contract(abi, contract_address);
In the front-end web app, you can use the contract object to call the functions of your Rust smart contract. For example, let's say you want to call the buy function when the user clicks on the "Buy" button. You can do this by adding an onclick event handler to the button element in the view function, like this:
Copy code
html! {
    <button onclick=self.link.callback(|_| Msg::Buy)>{ "Buy" }</button>
}
In the update function, handle the Buy message and call the buy function of the contract object.

fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::Buy => {
            let sender = (web3.eth.getAccounts())[0];
            contract.methods.buy().send({ from: sender });
        }
        // Other message cases go here
    }
    // Return true to indicate that the component should be re-rendered
    true
}
This is just a simple example of how you can use the contract object to call the functions of your Rust smart contract from the front-end web app. You can use similar logic to call other functions and pass in input parameters as necessary.
  how to interact with a Rust smart contract from a Rust front-end web app. 






*/


use yew::{html, Html, Component, ComponentLink, ShouldRender};
use yew::services::console::ConsoleService;
use yew::services::dialog::{DialogService, DialogRequest};
use yew::html::InputData;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::callback::Callback;
use yew::services::storage::{Area, StorageService};
use serde::{Deserialize, Serialize};
use std::fs;
use web3::types::{Address, Bytes};

// Define a struct to represent the state of the component
struct Model {
    // The ConsoleService is used to log messages to the browser's console
    console: ConsoleService,
    // The DialogService is used to display dialog boxes to the user
    dialog: DialogService,
    // The FetchService is used to send HTTP requests
    fetch: FetchService,
    // The IntervalService is used to execute a callback function at regular intervals
    interval: IntervalService,
    // The StorageService is used to access the browser's local storage
    storage: StorageService,
    // The Web3 object is used to interact with the Ethereum blockchain
    web3: web3::Web3<web3::transports::Http>,
    // The contract object is used to interact with the saffron vendor contract
    contract: Option<web3::contract::Contract<web3::transports::Http>>,
    // The contract address
    contract_address: Address,
    // The contract ABI (Application Binary Interface)
    contract_abi: Vec<web3::contract::ContractAbi>,
    // The current user's Ethereum address
    user_address: Option<Address>,
    // State variable to track whether the lightbox is open
    show_lightbox: bool,
    // State variable to track the type of lightbox being displayed
    lightbox_type: Option<LightboxType>,
    // State variable to track the quantity entered by the user when purchasing saffron
    quantity: u32,
    // State variable to track the name entered by the user when redeeming saffron
    name: String,
    // State variable to track the address entered by the user when redeeming saffron
    address: String,
    // State variable to track the quantity entered by the user when redeeming saffron
    redeem_quantity: u32,
    // State variable to track the current balance of the contract
    contract_balance: Option<web3::types::U256>,
    // State variable to track the current user balance
    user_balance: Option<web3::types::U256>,
    // State variable to track the FetchTask for fetching the contract balance
    balance_task: Option<FetchTask>,
    // State variable to track the IntervalTask for updating the contract and user balances
    balance_interval: Option<IntervalTask>,
}

// // Define an enum to represent the possible types of lightboxes that can be displayed
enum LightboxType {
    Buy,
    Redeem,
}

// Define an enum to represent the possible messages that the component can receive
enum Msg {
    // Message to toggle the lightbox, with an optional lightbox type
    ToggleLightbox(Option<LightboxType>),
    // Message to set the quantity state variable
    SetQuantity(String),
    // Message to set the name state variable
    SetName(String),
    // Message to set the address state variable
    SetAddress(String),
    // Message to set the redeem_quantity state variable
    SetRedeemQuantity(String),
    // Message to confirm a purchase
    Confirm,
    // Message to confirm a redemption
    ConfirmRedeem,
    // Message to update the contract and user balances
    UpdateBalances,
}

// Implement the Component trait for the Model struct
impl Component for Model {
    // Define the type of message that the component can receive
    type Message = Msg;
    // The component has no properties
    type Properties = ();

    // The create method is called when the component is first initialized
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Initialize the ConsoleService
        let console = ConsoleService::new();
        // Initialize the DialogService
        let dialog = DialogService::new();
        // Initialize the component state
        Model { console, dialog, show_lightbox: false, lightbox_type: None, quantity: 0, name: "".to_string(), address: "".to_string(), redeem_quantity: 0 }
    }
// Define a function to render the lightbox
fn view_lightbox(&self) -> Html {
    let content = match self.lightbox_type {
        Some(LightboxType::Buy) => {
            html! {
                <div class="lightbox-content">
                    <form>
                        <label for="quantity" class="form-label">{ "Quantity:" }</label>
                        <input type="number" id="quantity" class="form-input" value=self.quantity.to_string() oninput=self.link.callback(|e: InputData| Msg::SetQuantity(e.value)) />
                        <button type="button" class="btn btn-green" onclick=self.link.callback(|_| Msg::Confirm)>{ "Buy" }</button>
                    </form>
                </div>
            }
        }
        Some(LightboxType::Redeem) => {
            html! {
                <div class="lightbox-content">
                    <form>
                        <label for="name" class="form-label">{ "Name:" }</label>
                        <input type="text" id="name" class="form-input" value=self.name oninput=self.link.callback(|e: InputData| Msg::SetName(e.value)) />
                        <label for="address" class="form-label">{ "Address:" }</label>
                        <input type="text" id="address" class="form-input" value=self.address oninput=self.link.callback(|e: InputData| Msg::SetAddress(e.value)) />
                        <label for="redeem_quantity" class="form-label">{ "Quantity:" }</label>
                        <input type="number" id="redeem_quantity" class="form-input" value=self.redeem_quantity.to_string() oninput=self.link.callback(|e: InputData| Msg::SetRedeemQuantity(e.value)) />
                        <button type="button" class="btn btn-red" onclick=self.link.callback(|_| Msg::ConfirmRedeem)>{ "Redeem" }</button>
                    </form>
                </div>
            }
        }
        None => {
            html! {}
        }
    };


    if self.show_lightbox {
        html! {
            <div class="lightbox-overlay">
                { content }
            </div>
        }
    } else {
        html! {}
    }
}
fn view(&self) -> Html {
    // Define the lightbox content
    let lightbox = self.view_lightbox();
    // Define the main content of the page
    html! {
        <div class="container">
            // Header
            <h1>{ "Saffron Vendor" }</h1>
            // Buy button
            <button type="button" class="btn btn-green" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Buy)))>{ "Buy" }</button>
            // Redeem button
            <button type="button" class="btn btn-red" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Redeem)))>{ "Redeem" }</button>
            // Lightbox
            { lightbox }
        </div>
    }
}



    // The update method is called whenever the component's state needs to be updated
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // Match on the incoming message
        match msg {
            // If the message is to toggle the lightbox...
            Msg::ToggleLightbox(lightbox_type) => {
                // Update the show_lightbox state variable based on whether a lightbox type was provided
                self.show_lightbox = lightbox_type.is_some();
                // Update the lightbox_type state variable with the provided lightbox type
                self.lightbox_type = lightbox_type;
                // Return true to indicate that the component should be re-rendered
                true
            }
            // If the message is to set the quantity state variable...
            Msg::SetQuantity(quantity) => {
                // Parse the quantity string as a u32 and update the state variable
                self.quantity = quantity.parse().unwrap();
                // Return false to indicate that the component does not need to be re-rendered
                false
            }
            // If the message is to set the name state variable...
            Msg::SetName(name) => {
                // Update the name state variable with the provided name
                self.name = name;
                // Return false to indicate that the component does not need to be re-rendered
                false
            }
            // If the message is to set the address state variable...
            Msg::SetAddress(address) => {
                // Update the address state variable with the provided address
                self.address = address;
                // Return false to indicate that the component does not need to be re-rendered
                false
            }
            // If the message is to set the redeem_quantity state variable...
            Msg::SetRedeemQuantity(redeem_quantity) => {
            self.redeem_quantity = redeem_quantity.parse().unwrap();
            false
        }
        



//? Maybe add receipt. Make sure buy qty and redeem qty are diff variables
//? Make sure buy and redeem have different confirmation buttons




            // If the message is to confirm a purchase...
            Msg::Confirm => {
                // Call the buy function
                buy(self.quantity); 
                // Display a dialog box to the user to confirm the purchase
                self.dialog.confirm("Thank you for your purchase!");
                // Reset the quantity state variable
                self.quantity = 0;
                // Close the lightbox
                self.show_lightbox = false;
                // Return true to indicate that the component should be re-rendered
                true
            }
            // If the message is to confirm a redemption...
            Msg::ConfirmRedeem => {
                // Call the redeem function
                redeem(self.name.clone(), self.address.clone(), self.redeem_quantity);
                // Display a dialog box to the user to confirm the redemption
                self.dialog.confirm("Thank you for your redemption!");
                // Reset the name, address, and redeem_quantity state variables
                self.name = "".to_string();
                self.address = "".to_string();
                self.redeem_quantity = 0;
                // Close the lightbox
                self.show_lightbox = false;
                // Return true to indicate that the component should be re-rendered
                true
            }
        }
    }
}




// Define the main function
// Define the main function
fn main() {
    // Set up the event listener for the "Buy" button
    let buy_button = document.get_element_by_id("buy_button").unwrap();
    let buy_closure = Closure::wrap(Box::new(move || {
        // Get the quantity input field
        let quantity_input = document.get_element_by_id("quantity").unwrap();
        let quantity: u32 = quantity_input.dyn_ref::<HTMLInputElement>().unwrap().value().parse().unwrap();

        // Buy the specified quantity of saffron
        let receipt = buy_saffron(quantity).unwrap();

        // Display the transaction receipt
        alert(&format!("Transaction receipt: {:?}", receipt));
    }) as Box<dyn FnMut()>);
    buy_button.add_event_listener_with_callback("click", buy_closure.as_ref().unchecked_ref()).unwrap();
    buy_closure.forget();

    // Set up the event listener for the "Redeem" button
    let redeem_button = document.get_element_by_id("redeem_button").unwrap();
    let redeem_closure = Closure::wrap(Box::new(move || {
        // Get the name input field
        let name_input = document.get_element_by_id("name").unwrap();
        let name = name_input.dyn_ref::<HTMLInputElement>().unwrap().value();

        // Get the address input field
        let address_input = document.get_element_by_id("address").unwrap();
        let address = address_input.dyn_ref::<HTMLInputElement>().unwrap().value();

        // Get the quantity input field
        let quantity_input = document.get_element_by_id("redeem_quantity").unwrap();
        let quantity: u32 = quantity_input.dyn_ref::<HTMLInputElement>().unwrap().value().parse().unwrap();

        // Redeem the specified quantity of saffron
        let receipt = redeem_saffron(name, address, quantity).unwrap();

        // Display the transaction receipt
        alert(&format!("Transaction receipt: {:?}", receipt));
    }) as Box<dyn FnMut()>);
    redeem_button.add_event_listener_with_callback("click", redeem_closure.as_ref().unchecked_ref()).unwrap();
    redeem_closure.forget();
}

// Define the encode_function_call function
// This function takes in a function name and an array of parameters, and returns the encoded function call data
// Define the encode_function_call function
// This function takes in a function name and an array of parameters, and returns the encoded function call data
fn encode_function_call(name: &[u8], params: &[Vec<u8>]) -> Vec<u8> {
    // Encode the function name
    let mut data = web3.eth.abi.encode_function_signature(name);

    // Encode the function parameters
    data.extend_from_slice(&web3.eth.abi.encode_abi(&params));

    // Return the encoded data
    data
}



// Define the buy function
// This function takes in a u32 quantity argument, which represents the number of units of saffron that the user is purchasing
fn buy(quantity: u32) {
    // Get the user's Ethereum address
    let user_address = get_user_address().unwrap();

    // Create a transaction object
    let tx = Transaction {
        // Set the "to" field to the contract address
        to: Some(contract_address.parse().unwrap()),
        // Set the "value" field to the quantity multiplied by the price per unit
        value: quantity as u128 * PRICE_PER_UNIT,
        // Set the "data" field to the encoded function call data
        data: Some(encode_function_call(b"buy", &[])),
        // Set the "gas" field to the maximum gas allowed for the transaction
        gas: Some(MAX_GAS),
        // Set the "gas_price" field to the minimum gas price allowed for the transaction
        gas_price: Some(MIN_GAS_PRICE),
        // ... other transaction fields
    };

    // Sign the transaction using the user's Ethereum address
    let signed_tx = sign_transaction(tx, user_address).unwrap();

    // Send the signed transaction to the Ethereum network
    send_transaction(signed_tx).unwrap();
}

// Define the redeem function
// This function takes in three arguments: a string name, a string address, and a u32 quantity
fn redeem(name: String, address: String, quantity: u32) {
    // Get the user's Ethereum address
    let user_address = get_user_address().unwrap();

    // Convert the name and address strings to byte arrays
    let name_bytes = name.as_bytes();
    let address_bytes = address.as_bytes();

    // Create a transaction object
    let tx = Transaction {
        // Set the "to" field to the contract address
        to: Some(contract_address.parse().unwrap()),
        // Set the "value" field to zero
        value: 0,
        // Set the "data" field to the encoded function call data
        data: Some(encode_function_call(b"redeem", &[name_bytes.to_vec(), address_bytes.to_vec()])),
        // Set the "gas" field to the maximum gas allowed for the transaction
        gas: Some(MAX_GAS),
        // Set the "gas_price" field to the minimum gas price allowed for the transaction
        gas_price: Some(MIN_GAS_PRICE),
        // ... other transaction fields
    };

    // Sign the transaction using the user's Ethereum address
    let signed_tx = sign_transaction(tx, user_address).unwrap();

    // Send the signed transaction to the Ethereum network
    send_transaction(signed_tx).unwrap();
}









// Define the get_user_address function
// This function returns the Ethereum address of the user who is currently logged in to their wallet
fn get_user_address() -> Result<Address, String> {
    // Call the web3.eth.getAccounts method to get the user's Ethereum address
    let accounts = web3.eth().accounts().map_err(|e| e.to_string())?;

    // Return the first element of the accounts array (the user's Ethereum address)
    Ok(accounts[0])
}

// Define the sign_transaction function
// This function takes in a transaction object and an Ethereum address, and returns the signed transaction
fn sign_transaction(tx: Transaction, address: Address) -> Result<SignedTransaction, String> {
    // Call the web3.eth.signTransaction method to sign the transaction
    let signed_tx = web3.eth().sign_transaction(tx, address).map_err(|e| e.to_string())?;

    // Return the signed transaction
    Ok(signed_tx)
}

// Define the send_transaction function
// This function takes in a signed transaction and sends it to the Ethereum network
fn send_transaction(signed_tx: SignedTransaction) -> Result<TransactionReceipt, String> {
    // Call the web3.eth.sendSignedTransaction method to send the signed transaction to the Ethereum network
    let receipt = web3.eth().send_signed_transaction(signed_tx).map_err(|e| e.to_string())?;

    // Return the transaction receipt
    Ok(receipt)
}



// Define the create_transaction function
// This function takes in a function name, an array of parameters, and a value (in Ether), and returns a transaction object
fn create_transaction(name: &[u8], params: &[Vec<u8>], value: u64) -> Transaction {
    // Encode the function call data
    let data = encode_function_call(name, params);

    // Create the transaction object
    let tx = Transaction {
        // Set the "to" field to the contract address
        to: Some(contract_address),
        // Set the "data" field to the encoded function call data
        data: Some(data),
        // Set the "value" field to the specified value (in Ether)
        value: value,
        // Set the "gas" field to the maximum gas allowed for the transaction
        gas: 100_000_000,
        // Set the "gas_price" field to the minimum gas price allowed for the transaction
        gas_price: 1_000_000_000,
    };

    // Return the transaction object
    tx
}

// Define the buy_saffron function
// This function sends a transaction to the Ethereum network to buy saffron
fn buy_saffron(quantity: u32) -> Result<TransactionReceipt, String> {
    // Get the user's Ethereum address
    let user_address = get_user_address()?;

    // Calculate the value (in Ether) of the saffron purchase
    let value = quantity as u64 * 100;

    // Create the transaction object
    let tx = create_transaction(b"buy", &[], value);

    // Sign the transaction
    let signed_tx = sign_transaction(tx, user_address)?;

    // Send the signed transaction to the Ethereum network
    let receipt = send_transaction(signed_tx)?;

    // Return the transaction receipt
    Ok(receipt)
}

// Define the redeem_saffron function
// This function sends a transaction to the Ethereum network to redeem saffron
fn redeem_saffron(name: String, address: String, quantity: u32) -> Result<TransactionReceipt, String> {
    // Get the user's Ethereum address
    let user_address = get_user_address()?;

    // Convert the name and address strings to byte arrays
    let name_bytes = web3.utils.from_ascii(name);
    let address_bytes = web3.utils.from_ascii(address);

    // Create the parameters array
    let params = [&name_bytes, &address_bytes];

    // Create the transaction object
    let tx = create_transaction(b"redeem", &params, 0);

    // Sign the transaction
    let signed_tx = sign_transaction(tx, user_address)?;

    // Send the signed transaction to the Ethereum network
    let receipt = send_transaction(signed_tx)?;

    // Return the transaction receipt
    Ok(receipt)
}















// The above is the most complete code rendered missing the rest of view 


    // The create method is called when the component is first initialized
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Initialize the ConsoleService
        let console = ConsoleService::new();
        // Initialize the DialogService
        let dialog = DialogService::new();
        // Initialize the FetchService
        let fetch = FetchService::new();
        // Initialize the IntervalService
        let interval = IntervalService::new();
        // Initialize the StorageService
        let storage = StorageService::new(Area::Local).expect("Error initializing StorageService");
        // Initialize the Web3 object with a HTTP transport
        let web3 = web3::Web3::new(web3::transports::Http::new("http://localhost:8545").expect("Error initializing HTTP transport"));
        // Initialize the contract address
        let contract_address = Address::from_slice(b"0x...");
        // Load the contract ABI from a JSON file
        let contract_abi = fs::read_to_string("contract.json").expect("Error reading contract ABI");
        let contract_abi: Vec<web3::contract::ContractAbi> = serde_json::from_str(&contract_abi).expect("Error parsing contract ABI");
        // Initialize the contract object with the ABI and address
        let contract = Some(web3::contract::Contract::new(web3.eth(), contract_address, contract_abi).expect("Error initializing contract object"));
        // Initialize the user address to None
        let user_address = None;


        // Initialize the component state

        // Initialize the component state
        Model { console, dialog, fetch, interval, storage, web3, contract, contract_address, contract_abi, user_address, show_lightbox: false, lightbox_type: None, quantity: 0, name: "".to_string(), address: "".to_string(), redeem_quantity: 0, contract_balance: None, user_balance: None, balance_task: None, balance_interval: None }
    }



/*
// Define the view function for the component
// This function describes the HTML structure of the component
fn view(&self) -> Html {
    // If the lightbox is open, render the lightbox component
    let lightbox = if self.show_lightbox {
        // If the lightbox type is "Buy"...
        if let Some(LightboxType::Buy) = self.lightbox_type {
            // Render the buy lightbox component
            html! {
                <div class="lightbox">
                    // Form for purchasing saffron
                    <form>
                        // Label for the quantity input
                        <label for="quantity" class="form-label">{ "Quantity:" }</label>
                        // Input element for quantity
                        <input type="number" id="quantity" class="form-input" value=self.quantity.to_string() oninput=self.link.callback(|e: InputData| Msg::SetQuantity(e.value)) />
                        // Button to confirm purchase
                        <button type="button" class="btn btn-primary" onclick=self.link.callback(|_| Msg::Confirm)>{ "Buy" }</button>
                        // Button to close the lightbox
                        <button type="button" class="btn btn-secondary" onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>{ "Cancel" }</button>
                    </form>
                </div>
            }
        }
        // If the lightbox type is "Redeem"...
        else if let Some(LightboxType::Redeem) = self.lightbox_type {
            // Render the redeem lightbox component
            html! {
                <div class="lightbox">
                    // Form for redeeming saffron
                    <form>
                        // Label for the name input
                        <label for="name" class="form-label">{ "Name:" }</label>
                        // Input element for name
                        <input type="text" id="name" class="form-input" value=&self.name oninput=self.link.callback(|e: InputData| Msg::SetName(e.value)) />
                        // Label for the address input
                        <label for="address" class="form-label">{ "Address:" }</label>
                        // Input element for address
                        <input type="text" id="address" class="form-input" value=&self.address oninput=self.link.callback(|e: InputData| Msg::SetAddress(e.value)) />
                        // Label for the redeem_quantity input
                        <label for="redeem_quantity" class="form-label">{ "Quantity:" }</label>
                        // Input element for redeem_quantity
                        <input type="number" id="redeem_quantity" class="form-input" value=self.redeem_quantity.to_string() oninput=self.link.callback(|e: InputData| Msg::SetRedeemQuantity(e.value)) />
                        // Button to confirm redemption
                        <button type="button" class="btn btn-primary" onclick







fn view(&self) -> Html {
    html! {
        <div class="container">
            <h1>{ "Saffron Vendor" }</h1>
            <div class="card">
                <img src="/saffron.jpg" alt="Saffron" class="card-img-top" />
                <div class="card-body">
                    <h5 class="card-title">{ "Saffron" }</h5>
                    <p class="card-text">{ "The finest quality saffron available. Perfect for cooking and home remedies." }</p>
                    <button type="button" class="btn btn-green" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Buy)))>{ "Buy" }</button>
                    <button type="button" class="btn btn-red" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Redeem)))>{ "Redeem" }</button>
                </div>
            </div>
            { self.view_lightbox() }
        </div>
    }
}*/
/*







    // The view method is called to render the component's HTML
    fn view(&self) -> Html {
        // Render the HTML for the component
        html! {
            // Container element
            <div class="container">
                // Header element
                <header class="header">
                    // Title element
                    <h1 class="title">{ "Saffron Vendor" }</h1>
                    // Nav element
                    <nav class="nav">
                        // Home link
                        <a class="nav-link" href="/">{ "Home" }</a>
                        // About link
                        <a class="nav-link" href="/about">{ "About" }</a>
                    </nav>
                </header>
                // Main element
                <main class="main">
                    // Hero element
                    <div class="hero">
                        // Image element
                        <img src="/images/saffron.jpg" alt="Saffron" class="hero-image" />
                        // Title element
                        <h2 class="hero-title">{ "Experience the finest saffron on the market" }</h2>
                        // Button element to open the buy lightbox
                        <button class="btn btn-green" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Buy)))>{ "Buy" }</button>
                        // Button element to open the redeem lightbox
                        <button class="btn btn-red" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Redeem)))>{ "Redeem" }</button>
                    </div>
                    // Section element
                    <section class="section">
                        // Title element
                        <h3 class="section-title">{ "What is saffron?" }</h3>
                        // Paragraph element
                        <p class="section-text">{ "Saffron is a spice derived from the flower of Crocus sativus, commonly known as the saffron crocus. It is widely used in cooking and has a unique, pungent flavor and aroma. It is also known for its bright yellow-orange color and is often used as a natural dye." }</p>
                    </section>
                </main>
                // Footer element
                <footer class="footer">
                    // Paragraph element
                    <p class="footer-text">{ "Copyright 2021 Saffron Vendor" }</p>
                </footer>
                // Render the lightbox component if show_lightbox is true
                { self.view_lightbox() }
            </div>
        }
    }

 
//???????????



        // The view_lightbox method is called to render the lightbox component
    fn view_lightbox(&self) -> Html {
        // Match on the lightbox_type state variable
        match &self.lightbox_type {
            // If the lightbox type is Buy...
            Some(LightboxType::Buy) => {
                // Render the HTML for the buy lightbox
                html! {
                    // Overlay element
                    <div class="overlay" onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>
                        // Modal element
                        <div class="modal">
                            // Title element
                            <h3 class="modal-title">{ "Buy Saffron" }</h3>
                            // Form element
                            <form onsubmit=self.link.callback(|e: FocusEvent| {
                                e.prevent_default();
                                Msg::Confirm
                            })>
                                // Label element
                                <label for="quantity" class="form-label">{ "Quantity:" }</label>
                                // Input element for quantity
                                <input type="number" id="quantity" class="form-input" value=self.quantity.to_string() oninput=self.link.callback(|e: InputData| Msg::SetQuantity(e.value)) />
                                // Button element to submit the form
                                <button type="submit" class="btn btn-green">{ "Confirm" }</button>
                            </form>
                            // Button element to close the lightbox
                            <button class="btn btn-red" onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>{ "Cancel" }</button>
                        </div>
                    </div>
                }
            }

//
        // If the lightbox type is Some(LightboxType::Redeem)...
        Some(LightboxType::Redeem) => {
            // Return the HTML for the redeem lightbox
            html! {
                // Container element for the lightbox
                <div class="lightbox-container">
                    // Inner container element for the lightbox
                    <div class="lightbox">
                        // Title element for the lightbox
                        <h2 class="lightbox-title







            // If the lightbox type is Redeem...
            Some(LightboxType::Redeem) => {
                // Render the HTML for the redeem lightbox
                html! {
                    // Overlay element
                    <div class="overlay" onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>
                        // Modal element
                        <div class="modal">
                            // Title element
                            <h3 class="modal-title">{ "Redeem Saffron" }</h3>
                                                        // Form element
                            <form onsubmit=self.link.callback(|e: FocusEvent| {
                                e.prevent_default();
                                Msg::ConfirmRedeem
                            })>
                                // Label element
                                <label for="name" class="form-label">{ "Name:" }</label>
                                // Input element for name
                                <input type="text" id="name" class="form-input" value=&self.name oninput=self.link.callback(|e: InputData| Msg::SetName(e.value)) />
                                // Label element
                                <label for="address" class="form-label">{ "Address:" }</label>
                                // Input element for address
                                <input type="text" id="address" class="form-input" value=&self.address oninput=self.link.callback(|e: InputData| Msg::SetAddress(e.value)) />
                                // Label element
                                <label for="redeem_quantity" class="form-label">{ "Quantity:" }</label>
                                // Input element for redeem_quantity
                                <input type="number" id="redeem_quantity" class="form-input" value=self.redeem_quantity.to_string() oninput=self.link.callback(|e: InputData| Msg::SetRedeemQuantity(e.value)) />
                                //

//?????????                                // Button element to submit the form
                                <button type="submit" class="btn btn-green">{ "Confirm" }</button>
                            </form>
                            // Button element to close the lightbox
                            <button class="btn btn-red" onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>{ "Cancel" }</button>
                        </div>
                    </div>
                }
            }
            // If the lightbox type is None (i.e. no lightbox should be displayed)...
            None => {
                // Return an empty HTML fragment
                html! {}
            }
        }
    }
  
}

// The main function is the entry point to the program
fn main() {
    // Initialize the Yew runtime
    yew::initialize();
    // Create a component instance
    let app = App::<Model>::new(());
    // Mount the component to the DOM
    app.mount_to_body();
    // Run the Yew runtime event loop
    yew::run_loop();
}
*/


/*This code defines a Rust component using the Yew framework. The component has a state that includes a flag to track whether a lightbox is currently being displayed, and an enum to track the type of lightbox that is being displayed (either Buy or Redeem). The component also has state variables to track the quantity of saffron being purchased, the name and address of the user redeeming saffron, and the quantity of saffron being redeemed.

The component has two buttons: a green "Buy Saffron" button and a red "Redeem Saffron" button. When either button is clicked, it opens a lightbox component with a form for the user to enter the necessary information. The "Buy Saffron" lightbox has a form with a field for the user to enter the quantity of saffron they want to purchase, and a "Confirm" button to submit the
form. When the "Confirm" button is clicked, it calls the `buy` function defined earlier, which sends a transaction to the smart contract's `buy` function.

The "Redeem Saffron" lightbox has a form with fields for the user to enter their name, address, and the quantity of saffron they want to redeem, as well as a "Confirm" button to submit the form. When the "Confirm" button is clicked, it calls the `redeem` function defined earlier, which sends a transaction to the smart contract's `redeem` function with the user's name and address as input parameters.

Both lightboxes also have a "Cancel" button to close the lightbox.

The component's `view` method is responsible for rendering the HTML for the component, including the buttons and the lightbox. The `view_lightbox` method is called to render the lightbox component, which is based on the value of the `lightbox_type` state variable. If the `lightbox_type` is `Some(LightboxType::Buy)`, it renders the "Buy Saffron" lightbox, and if it is `Some(LightboxType::Redeem)`, it renders the "Redeem Saffron" lightbox. If the `lightbox_type` is `None`, it renders an empty HTML fragment.

Finally, the `main` function initializes the Yew runtime, creates an instance of the component, and mounts it to the DOM. It then runs the Yew runtime event loop to handle events and updates to the component.
*/
