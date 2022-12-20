use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct RedeemForm {
    link: ComponentLink<Self>,
    name: String,
    email: String,
    quantity: String,
    error: Option<String>,
}

pub enum Msg {
    NameChanged(String),
    EmailChanged(String),
    QuantityChanged(String),
    Submit,
}

impl Component for RedeemForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        RedeemForm {
            link,
            name: "".to_string(),
            email: "".to_string(),
            quantity: "".to_string(),
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NameChanged(name) => self.name = name,
            Msg::EmailChanged(email) => self.email = email,
            Msg::QuantityChanged(quantity) => self.quantity = quantity,
            Msg::Submit => {
                self.error = None;
                // Validate the form input
                // If the input is valid, send a request to redeem the item
                // If the request is successful, display a success message
                // If the request fails, display an error message
            },
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <form onsubmit=|e| {
                e.prevent_default();
                Msg::Submit
            }>
                <div class="form-group">
                    <label for="name">{"Name"}</label>
                    <input
                        id="name"
                        type="text"
                        value=&self.name
                        oninput=|e| Msg::NameChanged(e.value)
                        class="form-control"
                    />
                </div>
<div class="form-group">
    <label for="email">{"Email Address"}</label>
    <input
        id="email"
        type="email"
        value=&self.email
        oninput=|e| Msg::EmailChanged(e.value)
        class="form-control"
    />
</div>
<div class="form-group">
    <label for="quantity">{"Quantity"}</label>
    <input
        id="quantity"
        type="number"
        value=&self.quantity
        oninput=|e| Msg::QuantityChanged(e.value)
        class="form-control"
    />
</div>
<button type="submit" class="btn btn-primary">{"Redeem"}</button>
{ self.error.as_ref().map(|e| <p class="error">{ e }</p> }).unwrap_or_else(|| html! {}) }
</form>
