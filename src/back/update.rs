use iced::{exit, Color, Task};

use crate::{back::words::random_word, ui::{Message, Page, WGuess}};

pub fn func(wguess: &mut WGuess, message: Message) -> Task<Message> {
    match message {
        Message::Start => {
            wguess.game.playing = true;
            wguess.game.word = random_word().chars().collect();
            wguess.game.current_pos = (0, 0);
            wguess.game.words = [[' '; 5]; 5];
            wguess.game.msg = (Color::TRANSPARENT, "".to_string());
            wguess.page = Page::Game;
        },
        Message::Exit => return exit::<Message>(),

        Message::InputChar(character) => {
            if !character.is_alphabetic() || !wguess.game.playing {
                return Task::none()
            }
  
            let (row, col) = wguess.game.current_pos;
            if col != 4{
                wguess.game.words[row as usize][col as usize] = character;
                wguess.game.current_pos = (row, col + 1);
            } else if wguess.game.words[row as usize][col as usize] == ' ' {
                wguess.game.words[row as usize][col as usize] = character;
            }

            return Task::none()
        },
        Message::RemoveChar => {
            if !wguess.game.playing {
                return Task::none()
            }

            let (row, col) = wguess.game.current_pos;
            if col > 0 {
                wguess.game.words[row as usize][col as usize - 1] = ' ';
                wguess.game.current_pos = (row, col - 1);
            }
        },
        Message::SubmitWord => {
            if !wguess.game.playing {
                return Task::none()
            }
            
            let (row, col) = wguess.game.current_pos;
            if col == 4 {
                // Check if the word is correct
                let word: String = wguess.game.words[row as usize].iter().collect::<String>();
                let word = word.chars().collect::<Vec<char>>();
                if word == wguess.game.word {
                    // Correct word
                    wguess.game.playing = false;
                    wguess.game.msg = (Color::from_rgb8(0, 255, 0), 
                        "Вы угадали слово!".to_string());
                } else {
                    wguess.game.current_pos = (row + 1, 0);
                }
            }
            
            if wguess.game.current_pos.0 == 5 {
                wguess.game.playing = false;
                wguess.game.msg = (Color::from_rgb8(255, 0, 0), 
                    "Вы не угадали слово!".to_string());
                return Task::none()
            }
        }
    }
    Task::none()
}