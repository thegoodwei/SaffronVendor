use web3::{
    contract::Contract, // Import the Contract struct from the contract module in the web3 crate
    types::{Address, Bytes}, // Import the Address and Bytes types from the types module in the web3 crate
    Web3, // Import the Web3 type from the web3 crate
};
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender}; // Import various types and functions from the yew crate

// Define a struct to represent the form component
pub struct Form {
    web3: Web3<Http>, // Declare a field to hold a Web3 client
    contract: Contract<Http>, // Declare a field to hold a contract instance
    name: String, // Declare a field to hold the name input
    address: String, // Declare a field to hold the address input
    wallet_connected: bool, // Declare a field to hold a flag indicating whether a wallet is connected
    onsubmit: Callback<(String, String)>, // Declare a field to hold a callback to be executed when the form is submitted
}

// Define a struct to hold the properties for the Form component
#[derive(Properties, Clone)]
pub struct FormProps {
    pub onsubmit: Callback<(String, String)>, // Declare a field to hold a callback to be executed when the form is submitted
}

// Implement the Component trait for the Form struct
impl Component for Form {
    type Message = (); // Define the message type as an empty tuple
    type Properties = FormProps; // Define the properties type as FormProps

    // Implement the create method to initialize the Form component
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Initialize the Web3 client and contract
        let web3 = Web3::new(Http::new("http://localhost:8545"));
        let contract = Contract::new(web3.eth(), Address::zero(), Bytes::default());

        // Check if a wallet is connected
        let wallet_connected = contract.is_wallet_connected().wait().unwrap();

        // Return a new instance of the Form struct with the fields initialized
        Self {
            web3,
            contract,
            name: String::new(),
            address: String::new(),
            wallet_connected,
            onsubmit: props.onsubmit,
        }
    }

    // Implement the update method to handle messages
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false // Return false to indicate that the view does not need to be re-rendered
    }

    // Implement the change method to update the component's properties
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsubmit = props.onsubmit; // Update the onsubmit field with the new value from the properties
        true // Return true to indicate that the component should be re-rendered
    }

    // Implement the view method to define the component's HTML
    fn view(&self) -> Html {
        if self.wallet_connected {
            // If a wallet is connected, render the form
            html! {
                <form onsubmit=self.handle_submit>

                    <label for="name">{"Name:"}</label>
                    <input id="name" type="text" value=&self.name oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value)) />
                    <br />
                    <label for="address">{"Address:"}</label>
                    <input id="address" type="text" value=&self.address oninput=self.link.callback(|e: InputData| Msg::UpdateAddress(e.value)) />
                    <br />
                    <button type="submit">{"Submit"}</button>
                </form>
            }
        } else {
            html! {
                <button onclick=self.handle_login_click>{"Log In with Ethereum"}</button>
            }
        }
    }
}

fn main() {
    yew::start_app::<Form>();
}
