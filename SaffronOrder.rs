use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use reqwest::{Client, Response};

#[derive(Serialize, Deserialize, Debug)]
struct SaffronOrder {
    x: u64,
    mail_address: String,
    order_number: u64,
}

async fn ask_mail_address() -> String {
    println!("Please enter a valid US mail address:");
    let mut mail_address = String::new();
    io::stdin().read_line(&mut mail_address).await.unwrap();
    mail_address.trim().to_string()
}

async fn gen_order_number() -> u64 {
    static mut ORDER_NUMBER: u64 = 0;
    unsafe {
        ORDER_NUMBER += 1;
        ORDER_NUMBER
    }
}

async fn redeem_saffron(x: u64, client: &Client) -> Result<Response, reqwest::Error> {
    let order = SaffronOrder {
        x,
        mail_address: ask_mail_address().await,
        order_number: gen_order_number().await,
    };

    let creds = "AWS_CREDS_PLACEHOLDER";
    let url = "https://fws.com/api/redeem_saffron";
    let res = client.post(url)
        .header("Authorization", creds)
        .json(&order)
        .send()
        .await;
    res
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let x = 100; // number of saffron tokens to redeem
    let res = redeem_saffron(x, &client).await;
    println!("Response: {:?}", res);

    Ok(())
}
