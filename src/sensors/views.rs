use crate::common::models::{AppState, Message};
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Fill};

pub(crate) fn view_sensors(state: &AppState) -> Element<crate::common::models::Message> {
    let header = row![
        text("ID").size(16),
        horizontal_space(),
        text("Serial Number").size(16),
        horizontal_space(),
        text("Name").size(16),
        horizontal_space(),
        text("Unit").size(16),
    ]
    .spacing(20);

    let rows: Vec<Element<Message>> = state
        .sensors
        .iter()
        .map(|sensor| {
            row![
                text(sensor.id.to_string()).size(14),
                horizontal_space(),
                text(sensor.serial_number.clone()).size(14),
                horizontal_space(),
                text(sensor.parameter.name.clone()).size(14),
                horizontal_space(),
                text(sensor.parameter.unit.clone()).size(14),
            ]
            .spacing(20)
            .into()
        })
        .collect();

    container(
        column![
            header,
            column(rows).spacing(10),
            button("Back").on_press(Message::BackToMain)
        ]
        .spacing(20),
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
