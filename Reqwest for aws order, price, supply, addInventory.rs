/*
The following code is for interacting with the Amazon Marketplace API.
It includes functions for checking the price and supply of a product,
ordering inventory, placing customer orders, and checking the status of an order.
It uses the reqwest library for making HTTPS requests and the serde library for
serializing and deserializing data.
*/

use serde::{Deserialize, Serialize}; // imports the Deserialize and Serialize traits from the serde library
use std::error::Error; // imports the Error trait from the std library
use reqwest::Client; // imports the Client type from the reqwest library
use reqwest::header::{HeaderMap, HeaderValue}; // imports the HeaderMap and HeaderValue types from the reqwest::header module

#[derive(Serialize, Deserialize)] // indicates that the Product struct should be serializable and deserializable
struct Product {
name: String, // field for the product name
price: f64, // field for the product price
quantity: u64, // field for the product quantity
}

#[derive(Serialize, Deserialize)] // indicates that the Order struct should be serializable and deserializable
struct Order {
product: Product, // field for the product being ordered
quantity: u64, // field for the quantity of the product being ordered
}

#[derive(Serialize, Deserialize)] // indicates that the OrderStatus struct should be serializable and deserializable
struct OrderStatus {
order_id: String, // field for the order id
status: String, // field for the order status
}

#[derive(Serialize, Deserialize)] // indicates that the Inventory struct should be serializable and deserializable
struct Inventory {
product_id: String, // field for the product id
quantity: u64, // field for the quantity of the product
}

// Async function to check the price of a product
async fn check_price(product_id: &str, api_key: &str) -> Result<f64, JsValue> {
let client = Client::new(); // creates a new HTTP client
let base_url = "https://marketplace.api.amazon.com/v1/products/"; // base URL for the products endpoint
// creates a request builder for a GET request to the products endpoint with the product id appended to the base URL
let mut builder = client.get(&format!("{}{}", base_url, product_id));
// adds the x-api-key header to the request with the provided API key
builder = builder.header("x-api-key", api_key);
// sends the request and assigns the response value to a variable
let resp_value = builder.send().await?;
let status = resp_value.status(); // gets the status code of the response
// if the status code is not a success code, return an error
if !status.is_success() {
return Err(JsValue::from_str(&format!("Request failed with status code: {}", status)));
}
// deserializes the response body as a Product struct and assigns it to a variable
let product: Product = resp_value.json().await?;
// returns the price field of the Product struct
Ok(product.price)}

// Async function to check the supply of a product
async fn check_supply(product_id: &str, api_key: &str) -> Result<u64, JsValue> {
let client = Client::new(); // creates a new HTTP client
let base_url = "https://marketplace.api.amazon.com/v1/inventory/"; // base URL for the inventory endpoint
// creates a request builder for a GET request to the inventory endpoint with the product id appended to the base URL
let mut builder = client.get(&format!("{}{}", base_url, product_id));
// adds the x-api-key header to the request with the provided API key
builder = builder.header("x-api-key", api_key);
// sends the request and assigns the response value to a variable
let resp_value = builder.send().await?;
let status = resp_value.status(); // gets the status code of the response
// if the status code is not a success code, return an error
if !status.is_success() {
return Err(JsValue::from_str(&format!("Request failed with status code: {}", status)));
}
// deserializes the response body as an Inventory struct and assigns it to a variable
let inventory: Inventory = resp_value.json().await?;
// returns the quantity field of the Inventory struct
Ok(inventory.quantity)
}

// Async function to order inventory for a product
async fn order_inventory(product_id: &str, quantity: u64, api_key: &str) -> Result<(), JsValue> {
let client = Client::new(); // creates a new HTTP client
let base_url = "https://marketplace.api.amazon.com/v1/inventory/"; // base URL for the inventory endpoint
// creates a request builder for a POST request to the inventory endpoint with the product id appended to the base URL
let mut builder = client.post(&format!("{}{}", base_url, product_id));
// adds the x-api-key header to the request with the provided API key
builder = builder.header("x-api-key", api_key);
// adds the quantity of the product to be ordered in the request body as an Inventory struct
builder = builder.json(&Inventory {
product_id: product_id.to_string(),
quantity: quantity,
});
// sends the request and assigns the response value to a variable
let resp_value = builder.send().await?;
let status = resp_value.status(); // gets the status code of the response
// if the status code is not a success code, return an error
if !status.is_success() {
return Err(JsValue::from_str(&format!("Request failed with status code: {}", status)));
}
// if the request is successful, return an empty tuple
Ok(())
}

// Async function to place a customer order for a product
async fn place_customer_order(product_id: &str, quantity: u64, api_key: &str) -> Result<String, JsValue



// Async function to place a customer order for a product
async fn place_customer_order(product_id: &str, quantity: u64, api_key: &str) -> Result<String, JsValue> {
let client = Client::new(); // creates a new HTTP client
let base_url = "https://marketplace.api.amazon.com/v1/orders/"; // base URL for the orders endpoint
// gets the product information using the provided product id and API key
let product = get_product(product_id, api_key).await?;
// creates an Order struct with the product and quantity of the product to be ordered
let order = Order {
product: product,
quantity: quantity,
};
// creates a request builder for a POST request to the orders endpoint with the product id appended to the base URL
let mut builder = client.post(&format!("{}{}", base_url, product_id));
// adds the x-api-key header to the request with the provided API key
builder = builder.header("x-api-key", api_key);
// adds the Order struct to the request body
builder = builder.json(&order);
// sends the request and assigns the response value to a variable
let resp_value = builder.send().await?;
let status = resp_value.status(); // gets the status code of the response
// if the status code is not a success code, return an error
if !status.is_success() {
return Err(JsValue::from_str(&format!("Request failed with status code: {}", status)));
}
// deserializes the response body as an OrderStatus struct and assigns it to a variable
let order_status: OrderStatus = resp_value.json().await?;
// returns the order_id field of the OrderStatus struct
Ok(order_status.order_id)
}

// Async function to check the status of an order
async fn check_order_status(order_id: &str, api_key: &str) -> Result<String, JsValue> {
let client = Client::new(); // creates a new HTTP client
let base_url = "https://marketplace.api.amazon.com/v1/orders/"; // base URL for the orders endpoint
// creates a request builder for a GET request to the orders endpoint with the order id appended to the base URL
let mut builder = client.get(&format!("{}{}", base_url, order_id));
// adds the x-api-key header to the request with the provided API key
builder = builder.header("x-api-key", api_key);
// sends the request and assigns the response value to a variable
let resp_value = builder.send().await?;
let status = resp_value.status(); // gets the status code of the response
// if the status code is not a success code, return an error
if !status.is_success() {
return Err(JsValue::from_str(&format!("Request failed with status code: {}", status)));
}
// deserializes the response body as an OrderStatus struct and assigns it to a variable


// Async function to check the status of an order
async fn check_order_status(order_id: &str, api_key: &str) -> Result<String, JsValue> {
let client = Client::new(); // creates a new HTTP client
let base_url = "https://marketplace.api.amazon.com/v1/orders/"; // base URL for the orders endpoint
// creates a request builder for a GET request to the orders endpoint with the order id appended to the base URL
let mut builder = client.get(&format!("{}{}", base_url, order_id));
// adds the x-api-key header to the request with the provided API key
builder = builder.header("x-api-key", api_key);
// sends the request and assigns the response value to a variable
let resp_value = builder.send().await?;
let status = resp_value.status(); // gets the status code of the response
// if the status code is not a success code, return an error
if !status.is_success() {
return Err(JsValue::from_str(&format!("Request failed with status code: {}", status)));
}
// deserializes the response body as an OrderStatus struct and assigns it to a variable
let order_status: OrderStatus = resp_value.json().await?;
// returns the status field of the OrderStatus struct
Ok(order_status.status)
}

/*
Notes:

The functions in this code rely on the reqwest library for making HTTP requests and the serde library for serializing and deserializing data.
The functions make use of the Amazon Marketplace API to perform various actions such as checking prices, checking inventory levels, placing orders, and checking order statuses.
Each function sends an HTTP request to a specific endpoint of the API and returns either the requested information or a result indicating success or failure.
It is important to note that the API requires an API key to be included in the requests as an "x-api-key" header.
*/

