fn lightbox(&self) -> Html {
    html! {
        <div class="lightbox">
            <div class="dashboard">
                {
                    match self.lightbox_type {
                        Some(LightboxType::Buy) => {
                            html! {
                                <>
                                    <h2>{"Saffron Purchase"}</h2>
                                    <p>{"Price: $14/gram"}</p>
                                    <p>{"Quantity Available: 500 grams"}</p>
                                    <label>{"Enter Quantity:"}</label>
                                    <input type="number" value=self.quantity oninput=self.link.callback(|e: InputData| Msg::SetQuantity(e.value)) />
                                    <button onclick=self.link.callback(|_| Msg::Confirm)>{"Confirm"}</button>
                                    <button onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>{"Cancel"}</button>
                                </>
                            }
                        }
                        Some(LightboxType::Redeem) => {
                            html! {
                                <>
                                    <h2>{"Saffron Redeem"}</h2>
                                    <label>{"Enter Name:"}</label>
                                    <input type="text" value=self.name oninput=self.link.callback(|e: InputData| Msg::SetName(e.value)) />
                                    <label>{"Enter Address:"}</label>
                                    <input type="text" value=self.address oninput=self.link.callback(|e: InputData| Msg::SetAddress(e.value)) />
                                    <label>{"Enter Quantity:"}</label>
                                    <input type="number" value=self.quantity oninput=self.link.callback(|e: InputData| Msg::SetQuantity(e.value)) />
                                    <button onclick=self.link.callback(|_| Msg::ConfirmRedeem)>{"Confirm"}</button>
                                    <button onclick=self.link.callback(|_| Msg::ToggleLightbox(None))>{"Cancel"}</button>
                                </>
                            }
                        }
                        None => html!{},
                    }
                }
            </div>
        </div>
    }
}
