use crate::common::models::{AppState, LoginStatus, Message};
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Fill};

// struct ButtonStyle;

// impl ButtonStyle {
//     const GREEN: iced::button::Style = iced::button::Style {
//         background: Some(iced::Color::from_rgb8(0x00, 0x80, 0x00)),
//         border_radius: 5.0,
//         text_color: iced::Color::WHITE,
//         ..iced::button::Style::default()
//     };
// }

pub fn view_main(state: &AppState) -> Element<Message> {
    container(
        column![
            row![
                horizontal_space(),
                text("Welcome to the AstroRiver toolbox").size(20),
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
                    button(user_logged_in_identifier)
                        .on_press(Message::Logout)
                        .style(|_, _| {
                            iced::widget::button::Style {
                                background: Some(iced::Color::from_rgb8(0x00, 0x80, 0x00).into()), // Green
                                text_color: iced::Color::WHITE,
                                border: iced::Border::default(),
                                shadow: iced::Shadow::default(),
                            }
                        })
                } else if state.login_status == LoginStatus::LoggingIn {
                    button("Logging in").style(|_, _| {
                        iced::widget::button::Style {
                            background: Some(iced::Color::from_rgb8(0xFF, 0xFF, 0x00).into()), // Yellow
                            text_color: iced::Color::BLACK,
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        }
                    })
                } else {
                    button("Login").on_press(Message::Login).style(|_, _| {
                        iced::widget::button::Style {
                            background: Some(iced::Color::from_rgb8(0xFF, 0x00, 0x00).into()), // Red
                            text_color: iced::Color::WHITE,
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        }
                    })
                },
                horizontal_space(),
            ]
            .spacing(10),
            row![
                horizontal_space(),
                if !state.sensors.is_empty() {
                    Element::<Message>::from(
                        button("View Sensors").on_press(Message::ShowSensorsPage),
                    )
                } else {
                    Element::<Message>::from(button("View Sensors"))
                },
                horizontal_space(),
            ]
        ]
        .spacing(25),
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
