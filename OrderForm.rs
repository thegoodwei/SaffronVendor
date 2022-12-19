use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

pub struct Form {
    name: String,
    address: String,
    quantity: String,
    onsubmit: Callback<(String, String, String)>,
}

#[derive(Properties, Clone)]
pub struct FormProps {
    pub onsubmit: Callback<(String, String, String)>,
}

impl Component for Form {
    type Message = ();
    type Properties = FormProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            name: String::new(),
            address: String::new(),
            quantity: String::new(),
            onsubmit: props.onsubmit,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsubmit = props.onsubmit;
        true
    }

    fn view(&self) -> Html {
        html! {
            <form onsubmit=self.handle_submit>
                <label for="name">{"Name:"}</label>
                <input id="name" type="text" value=&self.name oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value)) />
                <br />
                <label for="address">{"Address:"}</label>
                <input id="address" type="text" value=&self.address oninput=self.link.callback(|e: InputData| Msg::UpdateAddress(e.value)) />
                <br />
                <label for="quantity">{"Quantity:"}</label>
                <input id="quantity" type="number" value=&self.quantity oninput=self.link.callback(|e: InputData| Msg::UpdateQuantity(e.value)) />
                <br />
                <button type="submit">{"Submit"}</button>
            </form>
        }
    }
}

fn handle_submit((name, address, quantity): (String, String, String)) {
    // Do something with the name, address, and quantity here
    println!("Name: {}", name);
    println!("Address: {}", address);
    println!("Quantity: {}", quantity);
}

fn main() {
    yew::start_app::<Form>();
}
