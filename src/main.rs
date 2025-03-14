mod auth;
mod config;

use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Fill, Task};
// use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
// use jwt_simple::prelude::*;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Multiply,
    Login,
    LoginCompleted(Result<String, String>),
    Logout,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

#[derive(Default)]
struct AppState {
    counter: Counter,
    login_token: Option<String>,
    login_status: LoginStatus,
    login_payload: Option<Claims>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoginStatus {
    LoggedOut,
    LoggingIn,
    LoggedIn,
}

impl Default for LoginStatus {
    fn default() -> Self {
        LoginStatus::LoggedOut
    }
}

#[tokio::main]
async fn main() -> iced::Result {
    iced::run("AstroRiver", update, view)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    auth_time: usize,
    sub: String,
    email: String,
    family_name: String,
    given_name: String,
    preferred_username: String,
    scope: String,
    // Add other claims as needed
}

fn extract_jwt_payload(token: &str) -> Result<Claims, Box<dyn Error>> {
    // Split by . and decode the base64 payload
    use base64::prelude::*;
    let payload = token.split('.').nth(1).ok_or("Invalid JWT")?;
    let payload = BASE64_URL_SAFE_NO_PAD.decode(payload.as_bytes())?;
    let payload = String::from_utf8(payload)?;
    let claims: Claims = serde_json::from_str(&payload)?;
    Ok(claims)
}

fn update(state: &mut AppState, message: Message) -> iced::Task<Message> {
    match message {
        Message::Increment => {
            state.counter.value += 1;
            Task::none()
        }
        Message::Multiply => {
            state.counter.value *= 2;
            Task::none()
        }
        Message::Login => {
            state.login_status = LoginStatus::LoggingIn;
            Task::perform(auth::login_flow(), Message::LoginCompleted)
        }
        Message::LoginCompleted(result) => match result {
            Ok(token) => {
                state.login_token = Some(token.clone());
                state.login_status = LoginStatus::LoggedIn;
                state.login_payload = extract_jwt_payload(&token)
                    .map_err(|e| eprintln!("Failed to extract JWT: {:?}", e))
                    .ok();
                // state.login_payload = token_payload.ok();

                Task::none()
            }
            Err(e) => {
                eprintln!("Login failed: {:?}", e);
                state.login_token = None;
                state.login_status = LoginStatus::LoggedOut;
                Task::none()
            }
        },
        Message::Logout => {
            state.login_token = None;
            state.login_status = LoginStatus::LoggedOut;
            Task::none()
        }
    }
}
fn view(state: &AppState) -> Element<Message> {
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
                    text(format!("Value: {}", state.counter.value)).size(20),
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
                row![
                    horizontal_space(),
                    if state.login_token.is_some() {
                        let user_logged_in_identifier = state
                            .login_payload
                            .as_ref()
                            .map(|claims| claims.email.as_str())
                            .unwrap_or("Logout");
                        button(user_logged_in_identifier).on_press(Message::Logout)
                    } else {
                        button("Login").on_press(Message::Login)
                    },
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
