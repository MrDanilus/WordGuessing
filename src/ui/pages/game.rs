use iced::{alignment::{Horizontal, Vertical}, border::Radius, widget::{button, column, container, row, text, text_input, Column, Row}, Border, Color, Element, Length, Padding};

use crate::ui::{Message, WGuess};

pub fn func(wguess: &WGuess) -> Element<Message> {
    let mut rows = vec![];
    for word_row in wguess.game.words{
        let mut res_row = vec![];
        for char in word_row{
            res_row.push(
                container(
                text_input("", &char.to_string())
                    .width(80).size(50).style(|_, _| text_input_style())
            ).into());
        }
        rows.push(
            Row::from_vec(res_row)
                .spacing(2).into()
        );
    };

    container(
        container(
            Column::from_vec(rows).spacing(2)
        ).align_x(Horizontal::Center)
        .width(Length::Fill).padding(Padding::top(Padding::default(), 20))
    ).into()
}

fn text_input_style() -> text_input::Style{
    text_input::Style { 
        background: iced::Background::Color(Color::from_rgb8(20, 20, 20)), 
        border: Border { 
            color: Color::from_rgb8(30, 30, 30), 
            width: 3.0,
            radius: Radius::from(10)
        },
        icon: Color::TRANSPARENT,
        placeholder: Color::TRANSPARENT,
        value: Color::WHITE,
        selection: Color::TRANSPARENT
    }
}