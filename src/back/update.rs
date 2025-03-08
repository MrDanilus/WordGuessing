use iced::{exit, Task};

use crate::{back::words::random_word, ui::{Message, Page, WGuess}};

pub fn func(wguess: &mut WGuess, message: Message) -> Task<Message> {
    match message {
        Message::Start => {
            wguess.game.playing = true;
            wguess.game.word = random_word().chars().collect();
            wguess.page = Page::Game;
        },
        Message::Exit => return exit::<Message>(),

        Message::InputChar(character) => {
            
            return Task::none()
        }
    }
    Task::none()
}