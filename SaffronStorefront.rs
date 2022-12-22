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

    // The update method is called whenever the component's state needs to be updated
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // Match on the incoming message
        match msg {
            // If the message is to toggle the lightbox...
            Msg::ToggleLightbox(lightbox_type) => {
                // Update the show_lightbox state variable based on whether a lightbox type was provided
                self.show_lightbox = lightbox_type.is_some();
                // Update the lightbox_type state variable with the provided lightbox type
                self.lightbox_type = lightbox_

        ///?????

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
                        <
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

//?????????

                                // Button element to submit the form
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

    // The view method is called to render the component
    fn view(&self) -> Html {
        html! {
            // Container element
            <div class="container">
                // Title element
                <h1 class="title">{ "Saffron Vendor" }</h1>
                // Paragraph element
                <p class="description">{ "Welcome to the Saffron Vendor website! Here you can buy and redeem high-quality saffron. Simply click the buttons below to get started." }</p>
                // Button element to open the buy lightbox
                <button class="btn btn-green" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Buy)))>{ "Buy Saffron" }</button>
                // Button element to open the redeem lightbox
                <button class="btn btn-red" onclick=self.link.callback(|_| Msg::ToggleLightbox(Some(LightboxType::Redeem)))>{ "Redeem Saffron" }</button>
                // Call the view_lightbox method to render the lightbox component
                { self.view_lightbox() }
            </div>
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



/*
*/
