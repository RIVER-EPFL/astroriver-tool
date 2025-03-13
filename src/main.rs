use dotenvy::dotenv;
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Fill};
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, DeviceAuthorizationResponse,
    DeviceAuthorizationUrl, EmptyExtraDeviceAuthorizationFields, PkceCodeChallenge, RedirectUrl,
    Scope, StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Multiply,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub keycloak_client_id: String,
    pub keycloak_url: String,
    pub keycloak_realm: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Load from .env file if available

        println!("{:?}", env::vars().collect::<Vec<(String, String)>>());

        Config {
            keycloak_client_id: env::var("KEYCLOAK_CLIENT_ID")
                .expect("KEYCLOAK_CLIENT_ID must be set"),
            keycloak_url: env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set"),
            keycloak_realm: env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set"),
        }
    }
}

fn device_flow_token() -> Result<String, Box<dyn Error>> {
    // Setup URLs and client as before

    let config = Config::from_env();
    let auth_url = AuthUrl::new(
        format!(
            "{}/realms/RIVER/protocol/openid-connect/auth",
            config.keycloak_url
        )
        .to_string(),
    )?;
    let token_url = TokenUrl::new(
        format!(
            "{}/realms/RIVER/protocol/openid-connect/token",
            config.keycloak_url
        )
        .to_string(),
    )?;
    let device_auth_url = DeviceAuthorizationUrl::new(
        format!(
            "{}/realms/RIVER/protocol/openid-connect/auth/device",
            config.keycloak_url
        )
        .to_string(),
    )?;

    let client = BasicClient::new(ClientId::new(config.keycloak_client_id.to_string()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_device_authorization_url(device_auth_url);

    let http_client = reqwest::blocking::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Request device code and verification URI
    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .add_scope(Scope::new("read".to_string()))
        .request(&http_client)?;

    // Build the verification URL properly with URL encoding.
    let mut verification_url = Url::parse(details.verification_uri().as_str())?;
    verification_url
        .query_pairs_mut()
        .append_pair("verification_uri_complete", details.user_code().secret());

    open::that(verification_url.as_str())?;
    println!(
        "Open this URL in your browser:\n{}",
        verification_url.as_str()
    );

    // Now we poll for the token
    println!("Waiting for user to authenticate...");
    loop {
        match client.exchange_device_access_token(&details).request(
            &http_client,
            std::thread::sleep,
            None,
        ) {
            Ok(token) => {
                println!("Access Token: {}", token.access_token().secret());
                return Ok(token.access_token().secret().to_owned());
            }
            Err(err) => {
                println!("Still waiting... ({})", err);
                sleep(Duration::from_secs(5)); // Wait before retrying
            }
        }
    }
}
pub fn main() -> iced::Result {
    let token: Option<String> = match device_flow_token() {
        Ok(token) => Some(token),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    };
    iced::run("AstroRiver", update, view)
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => {
            counter.value += 1;
        }
        Message::Multiply => {
            counter.value *= 2;
        }
    }
}

fn view(counter: &Counter) -> Element<Message> {
    container(
        column![
            column![
                row![
                    horizontal_space(),
                    text("Welcome to the AstroRiver toolbox").size(20),
                    horizontal_space(),
                ]
                .spacing(10),
                row![
                    // text("Cell 2").size(20),
                    horizontal_space(),
                    text(format!("Value: {}", counter.value)).size(20),
                    horizontal_space(),
                    // text("Cell 8").size(20),
                ]
                .spacing(10),
                row![
                    horizontal_space(),
                    button("Increment").on_press(Message::Increment),
                    button("Multiply").on_press(Message::Multiply),
                    horizontal_space(),
                ]
                .spacing(10),
            ]
            .spacing(25),
        ]
        .spacing(10),
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

// fn view(counter: &Counter) -> Element<Message> {
//     column![
//         text(counter.value).size(20),
//         button("Increment").on_press(Message::Increment),
//     ]
//     .spacing(10)
//     .into()
// }
