use iced::{exit, Color, Task};

use crate::{back::words::random_word, ui::{CharType, Message, Page, WGuess}};

pub fn func(wguess: &mut WGuess, message: Message) -> Task<Message> {
    match message {
        Message::Start => {
            wguess.game.playing = true;
            wguess.game.word = random_word().chars().collect();
            wguess.game.current_pos = (0, 0);
            wguess.game.words = [[(' ', CharType::NotFound); 5]; 6];
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
                wguess.game.words[row as usize][col as usize] = (character, CharType::NotFound);
                wguess.game.current_pos = (row, col + 1);
            } else if wguess.game.words[row as usize][col as usize].0 == ' ' {
                wguess.game.words[row as usize][col as usize] = (character, CharType::NotFound);
            }

            return Task::none()
        },
        Message::RemoveChar => {
            if !wguess.game.playing {
                return Task::none()
            }

            let (row, col) = wguess.game.current_pos;
            if col > 0 {
                wguess.game.words[row as usize][col as usize] = (' ', CharType::NotFound);
                wguess.game.current_pos = (row, col - 1);
            } else{
                wguess.game.words[row as usize][col as usize] = (' ', CharType::NotFound);
            }
        },
        Message::SubmitWord => {
            if !wguess.game.playing {
                return Task::none()
            }
            
            let (row, col) = wguess.game.current_pos;
            if col == 4 {
                let mut searched = Vec::new();
                let mut count_correct = 0;
                let word = wguess.game.word.clone();
                // Перебор букв в словах
                for i in 0..wguess.game.words[row as usize].len(){
                    let correct_char = word[i as usize];
                    let curr_char = wguess.game.words[row as usize][i as usize];
                    if curr_char.0 == correct_char{
                        count_correct += 1;
                        wguess.game.words[row as usize][i as usize].1 = CharType::Correct;
                    } else if word.contains(&curr_char.0) && !searched.contains(&curr_char.0){
                        wguess.game.words[row as usize][i as usize].1 = CharType::Exists;
                    }
                    searched.push(curr_char.0);
                }
                // TODO: добавить поддержку подсвечивания нескольких одинаковых букв в слове
                if count_correct == 5 {
                    // Correct word
                    wguess.game.playing = false;
                    wguess.game.msg = (Color::from_rgb8(0, 255, 0), 
                        "Вы угадали слово!".to_string());
                } else {
                    wguess.game.current_pos = (row + 1, 0);
                }
            }
            
            if wguess.game.current_pos.0 == wguess.game.words.len() as u8 {
                wguess.game.playing = false;
                wguess.game.msg = (Color::from_rgb8(255, 0, 0), 
                    format!("Вы не угадали слово!\nПравильное слово: '{}'", 
                    wguess.game.word.clone().into_iter().collect::<String>()));
                return Task::none()
            }
        }
    }
    Task::none()
}