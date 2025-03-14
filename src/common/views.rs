use crate::common::models::{AppState, Message};
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Fill};

pub fn view_main(state: &AppState) -> Element<Message> {
    container(
        column![
            row![
                horizontal_space(),
                text("Welcome to the AstroRiver toolbox").size(20),
                if !state.sensors.is_empty() {
                    Element::<Message>::from(
                        button("View Sensors").on_press(Message::ShowSensorsPage),
                    )
                } else {
                    Element::<Message>::from(button("View Sensors"))
                },
                horizontal_space(),
            ]
            .spacing(10),
            row![
                horizontal_space(),
                text(format!("Value: {}", state.counter.value)).size(20),
                horizontal_space(),
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
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
