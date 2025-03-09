use iced::{alignment::Horizontal, border::Radius, widget::{button, column, container, text, text_input, Column, Row}, Border, Color, Element, Length, Padding};

use crate::ui::{CharType, Message, WGuess};

pub fn func(wguess: &WGuess) -> Element<Message> {
    let mut rows = vec![];
    for word_row in 0..wguess.game.words.len(){
        let mut res_row = vec![];
        for char in 0..wguess.game.words[word_row].len(){
            res_row.push(
                container(
                text_input("", &wguess.game.words[word_row][char].0.to_string())
                    .width(80).size(50).style(move |_, _| 
                        text_input_style(word_row as u8, wguess.game.current_pos.0, 
                            wguess.game.words[word_row][char].1.clone())
                    )
            ).into());
        }
        rows.push(
            Row::from_vec(res_row)
                .spacing(2).into()
        );
    };

    container(
        column![
            //text(wguess.game.word.iter().collect::<String>()).size(30)
            //    .color(Color::WHITE).align_x(Horizontal::Center).width(Length::Fill),
            container(
                Column::from_vec(rows).spacing(2)
            ).align_x(Horizontal::Center)
            .width(Length::Fill).padding(Padding::top(Padding::default(), 20)),
            text(wguess.game.msg.1.clone()).size(30).width(Length::Fill)
                .color(wguess.game.msg.0).align_x(Horizontal::Center),
            container(
                button(text("Заново").size(28))
                    .on_press(Message::Start).width(160)
                    .padding(10)
            ).align_x(Horizontal::Center).width(Length::Fill)
            .padding(10)
        ]
    ).into()
}

fn text_input_style(row: u8, curr_row: u8, char_type: CharType) -> text_input::Style{
    let mut style = text_input::Style { 
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
    };
    match char_type {
        CharType::NotFound => {
            if row == curr_row{
                style.background = iced::Background::Color(Color::from_rgb8(30, 30, 30));
                style.border.color = Color::from_rgb8(40, 40, 40);
            }
        },
        CharType::Exists => {
            style.background = iced::Background::Color(Color::from_rgb8(227, 127, 10));
            style.border.color = Color::from_rgb8(207, 107, 0);
        },
        CharType::Correct => {
            style.background = iced::Background::Color(Color::from_rgb8(0, 120, 0));
            style.border.color = Color::from_rgb8(0, 100, 0);
        },
    }

    style
}