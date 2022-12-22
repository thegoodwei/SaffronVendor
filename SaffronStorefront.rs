//FRONTEND INIT BUY REDEEM ON ETHEREUM
// Import the Web3 library
import Web3 from "web3";

// Define the ABI (Application Binary Interface) of the smart contract
// This is used to generate the contract object and interact with its functions
const abi = [
    // The "buy" function is a payable function that takes no inputs and returns no outputs
    {
        "inputs": [], // No input parameters
        "name": "buy", // Function name
        "outputs": [], // No output parameters
        "stateMutability": "payable", // The function is payable (can receive Ether)
        "type": "function", // The function is a smart contract function
    },
    // The "redeem" function is a nonpayable function that takes two bytes32 inputs (name and address) and returns no outputs
    {
        "inputs": [
            {
                "name": "name", // Name of the first input parameter
                "type": "bytes" // The type of the first input parameter is bytes
            },
            {
                "name": "address", // Name of the second input parameter
                "type": "bytes" // The type of the second input parameter is bytes
            }
        ],
        "name": "redeem", // Function name
        "outputs": [], // No output parameters
        "stateMutability": "nonpayable", // The function is nonpayable (cannot receive Ether)
        "type": "function", // The function is a smart contract function
    },
    // ... other contract functions
];

// Define the contract address
const contract_address = "0x...";

// Initialize the Web3 provider
// This provider allows us to send Ethereum transactions and call smart contract functions
const web3 = new Web3(window.ethereum);

// Define the buy function
// Define the buy function (continued)
async function buy() {
    // Get the user's Ethereum address
    const address = (await web3.eth.getAccounts())[0];

    // Load the contract using the ABI and contract address
    // This generates a contract object that we can use to interact with the contract's functions
    const contract = new web3.eth.Contract(abi, contract_address);

    // Call the contract's "buy" function
    // This sends a transaction to the Ethereum network to call the function
    // The "from" parameter specifies the sender address
    await contract.methods.buy().send({ from: address });
}

// Define the redeem function
/* This function takes in two string arguments: name and address. These represent the name and delivery address of the user who is redeeming saffron.

The function first gets the Ethereum address of the user who is currently logged in to their wallet. This is done using the web3.eth.getAccounts method.

Next, the function creates a contract object using the ABI and contract address that were defined earlier. This object allows us to interact with the functions of the smart contract. */
async function redeem(name: String, address: String) {
    // Get the user's Ethereum address
    const sender = await web3.eth.getAccounts();

    // Create a contract object using the ABI and contract address
    const contract = new web3.eth.Contract(abi, contract_address);

    // Convert the name and address strings to byte arrays
    const name_bytes = web3.utils.fromAscii(name);
    const address_bytes = web3.utils.fromAscii(address);

    // Call the contract's "redeem" function
    // This sends a transaction to the Ethereum network to call the function
    // The "from" parameter specifies the sender address
    // The "name" and "address" parameters are the inputs to the function
    await contract.methods.redeem(name_bytes, address_bytes).send({ from: sender });
}


   
// BUTTONS FOR BUY AND REDEEM
/*
<!-- Import the Web3 library -->
<script src="https://cdn.jsdelivr.net/npm/web3@latest/dist/web3.min.js"></script>

<!-- Define the buy and redeem functions -->
<script>
  async function buy() {
    // Get the user's Ethereum address
    const sender = await web3.eth.getAccounts();

    // Create a contract object using the ABI and contract address
    const contract = new web3.eth.Contract(abi, contract_address);

    // Call the contract's "buy" function
    // This sends a transaction to the Ethereum network to call the function
    // The "from" parameter specifies the sender address
    await contract.methods.buy().send({ from: sender });
  }

  async function redeem(name: String, address: String) {
    // Get the user's Ethereum address
    const sender = await web3.eth.getAccounts();

    // Create a contract object using the ABI and contract address
    const contract = new web3.eth.Contract(abi, contract_address);

    // Convert the name and address strings to byte arrays
    const name_bytes = web3.utils.fromAscii(name);
    const address_bytes = web3.utils.fromAscii(address);

    // Call the contract's "redeem" function
    // This sends a transaction to the Ethereum network to call the function
    // The "from" parameter specifies the sender address
    // The "name" and "address" parameters are the inputs to the function
    await contract.methods.redeem(name_bytes, address_bytes).send({ from: sender });
  }
</script>

<!-- Define the buy button -->
<button onclick="buy()">Buy</button>

<!-- Define the redeem button -->
<button onclick="redeem('John Smith', '123 Main St')">Redeem</button>










//VISUAL INTERFACE 

/*This is the complete Rust code for the visually sleek Yew webpage with a UX for a saffron vendor. This webpage has two buttons: the green buy button opens a lightbox to reveal a price / available dashboard, a secure input for a Quantity, and 'confirm'; The red redeem button opens a different lightbox and another dashboard, with secure input for Name, Address, and Quantity. */
use yew::{html, Html, Component, ComponentLink, ShouldRender};
use yew::services::console::consoleService;
use yew::services::dialog::{DialogService, DialogRequest};
use yew::html::InputData;
use std::fs;

// Define a struct to represent the state of the component
struct Model {
    // The ConsoleService is used to log messages to the browser's console
    console: ConsoleService,
    // The DialogService is used to display dialog boxes to the user
    dialog: DialogService,
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
}

// Define an enum to represent the possible types of lightboxes that can be displayed
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
                // Parse the redeem_quantity string as a u32 and update the state variable
                self.redeem_quantity = redeem_quantity.parse().unwrap();
                // Return false to indicate that the component does not need to be re-rendered
                false
            }
            // If the message is to confirm a purchase...
            Msg::Confirm => {
                // Log a message to the console
                self.console.log(&format!("Confirmed purchase of saffron - Quantity: {}", self.quantity));
                // Request a dialog box with a message thanking the user for their purchase
                self.dialog.request(
                    DialogRequest::Alert("Thank you for your purchase!".to_string()),
                    link.callback(|_| Msg::ToggleLightbox(None)),               );
                // Return false to indicate that the component does not need to be re-rendered
                false
            }
        }
    }

    // The view method is called to render the component to the screen
    fn view(&self) -> Html {
        html! {
            <div id="saffron-vendor-home">
                <button onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Buy)))>
                    {"Buy Saffron"}
                </button>
                <button onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Redeem)))>
                    {"Redeem Saffron"}
                </button>
                // Render the lightbox if it is open
                {self.view_lightbox()}
            </div>
        }
    }
}

// Define a helper method to render the lightbox
impl Model {
    fn view_lightbox(&self) -> Html {
        // Match on the lightbox_type state variable to determine which lightbox to render
        match self.lightbox_type {
            // If the lightbox_type is Buy...
            Some(LightboxType::Buy) => self.view_buy_lightbox(),
            // If the lightbox_type is Redeem...
            Some(LightboxType::Redeem) => self.view_redeem_lightbox(),
            // If the lightbox_type is None, return an empty Html node
            None => html! {},
        }
    }

    // Define a helper method to render the buy lightbox
    fn view_buy_lightbox(&self) -> Html {
        html! {
            <div class="lightbox">
                <h2>{"Buy Saffron"}</h2>
                <label>{"Quantity: "}</label>
                // Render an input field for the quantity, with an oninput callback to update the state
                <input type="number" value=self.quantity oninput=self.link.callback(|e: InputData| Msg::SetQuantity(e.value)) />
                <button onclick=self.link.callback(|_| Msg::Confirm)>{"Confirm"}</button>
                <button onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>{"Cancel"}</button>
            </div>
        }
    }

    // Define a helper method to render the redeem lightbox
    fn view_redeem_lightbox(&self) -> Html {
        html! {
            <div class="lightbox">
                <h2>{"Redeem Saffron"}</h2>
                <label>{"Name: "}</label>
                // Render an input field for the name, with an oninput callback to update the state
                <input type="text" value=self.name oninput=self.link.callback(|e: InputData| Msg::SetName(e.value)) />
                <br />
                <label>{"Address: "}</label>
                // Render an input field for the address, with an oninput callback to update the state
                <button onclick=self.link.callback(|_| Msg::ConfirmRedeem)>{"Confirm"}</button>
                <button onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>{"Cancel"}</button>
            </div>
        }
    }
}
