use std::sync::Arc;

use iced::{theme::{Custom, Palette}, Color, Theme};

mod styles;
pub mod view;
mod pages;

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Exit,

    InputChar(char),
    RemoveChar,
    SubmitWord
}

#[derive(Debug, Clone, Copy, Default)]
pub enum CharType {
    #[default]
    NotFound,
    Exists,
    Correct
}
#[derive(Default)]
pub struct Game {
    pub playing: bool,
    pub word: Vec<char>,

    pub current_pos: (u8, u8), // (Row, Col)
    pub words: [[(char, CharType); 5]; 6],

    pub msg: (Color, String)
}
#[derive(Debug, Clone, Default)]
pub enum Page {
    #[default]
    Menu,
    Game
}
#[derive(Default)]
pub struct WGuess {
    pub page: Page,
    pub game: Game
}

impl WGuess {
    pub fn theme(&self) -> Theme{
        Theme::Custom(
            Arc::new(Custom::new(String::from("Dark"), Palette{
                background: Color::from_rgb8(0, 9, 24),
                text: Color::from_rgb(1.0, 1.0, 1.0),
                primary: Self::primary_color(),
                success: Self::primary_color(),
                danger: Self::primary_color()
            }))
        )
    }
}