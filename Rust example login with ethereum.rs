#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate web3;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;
use web3::futures::Future;

// Struct to hold login form data
#[derive(FromForm)]
struct LoginForm {
    address: String,
    password: String,
}

// Struct to hold Ethereum client and session data
struct EthereumData {
    client: web3::Web3<web3::transports::Http>,
    session: String,
}

// Display the login form
#[get("/login")]
fn login_form() -> Template {
    let context = Context::new();
    Template::render("login", &context)
}

// Process the login form submission
#[post("/login", data = "<login_form>")]
fn login(login_form: Form<LoginForm>, mut cookies: Cookies, ethereum: State<EthereumData>) -> Result<Redirect, Flash<Redirect>> {
    // Validate the login form data
    let form_data = login_form.get();
    if form_data.address.is_empty() || form_data.password.is_empty() {
        return Err(Flash::error(Redirect::to("/login"), "Invalid login credentials."));
    }

    // Attempt to sign in with the Ethereum wallet
    let sign_in = ethereum.client.eth().accounts().wait().map(|accounts| {
        // Check if the provided address is in the list of accounts
        if accounts.contains(&form_data.address) {
            // Generate a session token and set it as a cookie
            let session = generate_session_token();
            cookies.add_private(rocket::http::Cookie::new("session", session.clone()));

            // Save the session data in the EthereumData struct
            ethereum.session = session;
            Ok(())
        } else {
            Err(())
        }
    });

    match sign_in {
        Ok(Ok(())) => Ok(Redirect::to("/dashboard")),
        Ok(Err(())) => Err(Flash::error(Redirect::to("/login"), "Invalid login credentials.")),
        Err(err) => Err(Flash::error(Redirect::to("/login"), err.to_string())),
    }
}

// Check if the user is logged in and display the dashboard
#[get("/dashboard")]
fn dashboard(cookies: Cookies, ethereum: State<EthereumData>) -> Template {
    // Check if the user has a valid session cookie
    let session = cookies.get_private("session").and_then(|c| c.value().parse().ok());
    if let Some(session) = session {
        if session == ethereum.session {
            //User is logged in, render the dashboard template
let context = Context::new();
return Template::render("dashboard", &context);
}
}
// User is not logged in, redirect to login page
Flash::error(Redirect::to("/login"), "You must be logged in to access the dashboard.")
}

// Generate a random session token
fn generate_session_token() -> String {
// Generate a random 32-byte token and encode it as a hex string
let token: [u8; 32] = rand::random();
hex::encode(token)
}

fn main() {
// Initialize the Ethereum client
let client = web3::Web3::new(web3::transports::Http::new("https://mainnet.infura.io/v3/YOUR-API-KEY").unwrap());

// Initialize the EthereumData struct with the client and an empty session token
let ethereum = EthereumData {
    client,
    session: String::new(),
};

// Set up routes and start the server
rocket::ignite()
    .mount("/", routes![login_form,

