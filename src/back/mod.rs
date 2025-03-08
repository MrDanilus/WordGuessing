use iced::{event, keyboard::{self, Key}, Event, Subscription};

use crate::ui::{Message, WGuess};
pub mod words;
pub mod update;

impl WGuess{
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _windows| match event {
            Event::Keyboard(event) => {
                match event{
                    keyboard::Event::KeyReleased 
                        { key, location: _, modifiers: _ } => {
                            match key {
                                Key::Character(character) => {
                                    if character.len() == 1 {
                                        let char = character.chars().nth(0).unwrap();
                                        Some(Message::InputChar(char))
                                    } else{
                                        None
                                    }
                                },
                                _ => None
                            }
                        },
                    _ => None
                }
            },
            _ => None,
        })
    }
}