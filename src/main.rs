mod auth;
mod config;

use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Fill, Task};
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
    is_logging_in: bool,
}

#[tokio::main]
async fn main() -> iced::Result {
    iced::run("AstroRiver", update, view)
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
            state.is_logging_in = true;
            Task::perform(auth::login_flow(), Message::LoginCompleted)
        }
        Message::LoginCompleted(result) => {
            state.is_logging_in = false;
            match result {
                Ok(token) => {
                    state.login_token = Some(token.clone());
                    println!("Login successful: {}", token);
                    Task::none()
                }
                Err(e) => {
                    eprintln!("Login failed: {:?}", e);
                    state.login_token = None;
                    Task::none()
                }
            }
        }
        Message::Logout => {
            state.login_token = None;
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
                    button("Login").on_press(Message::Login),
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
