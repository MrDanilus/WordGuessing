use iced::{event, keyboard::{self, key::Named, Key}, Event, Subscription};

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
                                    if character.chars().count() == 1 {
                                        let char = character.chars().nth(0).unwrap();
                                        Some(Message::InputChar(char))
                                    } else{
                                        println!("Char: {:?}", character.len());
                                        None
                                    }
                                },
                                Key::Named(name) => {
                                    match name {
                                        Named::Backspace | Named::Delete => 
                                            Some(Message::RemoveChar),
                                        Named::Enter | Named::Space => 
                                            Some(Message::SubmitWord),
                                        _ => None
                                    }
                                }
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