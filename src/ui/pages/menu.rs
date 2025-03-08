use iced::{alignment::{Horizontal, Vertical}, border::Radius, widget::{button, column, container, text}, Border, Element, Length, Padding};

use crate::ui::{Message, WGuess};

pub fn func(_wguess: &WGuess) -> Element<Message> {
    container(
        column![
            text("Word Guessing").size(36).width(Length::Fill)
                .align_x(Horizontal::Center),
            container(
                column![
                    button(
                        text("Начать").size(28)
                    ).style(|_, status| button_style(status))
                        .on_press(Message::Start).width(160)
                        .padding(10),
                    button(
                        text("Выход").size(28)
                    ).style(|_, status| button_style(status))
                        .on_press(Message::Exit).width(160)
                        .padding(10)
                ].width(Length::Fill).align_x(Horizontal::Center)
                .spacing(10)
            ).align_y(Vertical::Center).width(Length::Fill).height(Length::Fill)
        ].padding(Padding::from([20, 0]))
    ).into()
}

fn button_style(status: button::Status) -> button::Style{
    let mut style = button::Style{
        background: None,
        text_color: WGuess::primary_color(),
        border: Border{
            color: WGuess::primary_color(),
            radius: Radius::new(10),
            width: 1.0
        },
        ..button::Style::default()
    };

    match status {
        button::Status::Hovered => {
            style.text_color = style.text_color.scale_alpha(0.7);
            style.border.color = style.border.color.scale_alpha(0.7);
        },
        button::Status::Pressed => {
            style.text_color = style.text_color.scale_alpha(0.4);
            style.border.color = style.border.color.scale_alpha(0.4);
        },
        _ => {}
    };
    style
}