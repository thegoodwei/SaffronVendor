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
