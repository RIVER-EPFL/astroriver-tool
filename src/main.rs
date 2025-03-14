mod auth;
mod common;
mod config;
mod sensors;

use crate::common::models::{AppState, Claims, LoginStatus, Message, Page};
use iced::{Element, Task};
use std::error::Error;

#[tokio::main]
async fn main() -> iced::Result {
    iced::run("AstroRiver", update, view)
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
                let token = state.login_token.clone().unwrap();
                Task::perform(
                    sensors::requests::request_sensors(token),
                    Message::SensorsRequested,
                )
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
        Message::SensorsRequested(result) => match result {
            Ok(sensors) => {
                println!("Sensors: {:?}", sensors);
                state.sensors = sensors;
                Task::none()
            }
            Err(e) => {
                eprintln!("Failed to request sensors: {:?}", e);
                Task::none()
            }
        },
        Message::ShowSensorsPage => {
            if !state.sensors.is_empty() {
                state.current_page = Page::Sensors;
            }
            Task::none()
        }
        Message::BackToMain => {
            state.current_page = Page::Main;
            Task::none()
        }
    }
}
fn view(state: &AppState) -> Element<Message> {
    match state.current_page {
        Page::Main => common::views::view_main(state),
        Page::Sensors => sensors::views::view_sensors(state),
    }
}
