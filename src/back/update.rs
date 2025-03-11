use iced::{exit, Color, Task};

use crate::{back::words::random_word, ui::{CharType, Message, Page, WGuess}};

use super::words::check_word;

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
  
            wguess.game.msg.1 = String::new();

            let (row, col) = wguess.game.current_pos;
            if col == wguess.game.word.len() as u8{
                return Task::none()
            }

            wguess.game.words[row as usize][col as usize] = (character, CharType::NotFound);
            wguess.game.current_pos = (row, col + 1);

            return Task::none()
        },
        Message::RemoveChar => {
            if !wguess.game.playing {
                return Task::none()
            }

            let (row, col) = wguess.game.current_pos;
            if col > 0 {
                let col = col - 1;
                wguess.game.words[row as usize][col as usize] = (' ', CharType::NotFound);
                wguess.game.current_pos = (row, col);
            } else{
                wguess.game.words[row as usize][col as usize] = (' ', CharType::NotFound);
            }
        },
        Message::SubmitWord => {
            if !wguess.game.playing {
                return Task::none()
            }
            
            let (row, col) = wguess.game.current_pos;
            let mut word: String = String::new();
            for char in wguess.game.words[row as usize]{
                word.push(char.0);
            }
            if !check_word(&word){
                wguess.game.playing = false;
                wguess.game.msg = (Color::from_rgb8(255, 0, 0), 
                    "Слово не найдено в словаре".to_string());
            }

            if col == wguess.game.word.len() as u8 {
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

                if count_correct == 5 {
                    // Correct word
                    wguess.game.playing = false;
                    wguess.game.msg = (Color::from_rgb8(0, 255, 0), 
                        "Вы угадали слово!".to_string());
                } else {
                    wguess.game.current_pos = (row + 1, 0);
                }
            } else{
                wguess.game.playing = false;
                wguess.game.msg = (Color::from_rgb8(255, 0, 0), 
                    "Некорректная длина слова".to_string());
            }
            
            if wguess.game.current_pos.0 == wguess.game.words.len() as u8 {
                wguess.game.msg = (Color::from_rgb8(255, 0, 0), 
                    format!("Вы не угадали слово!\nПравильное слово: '{}'", 
                    wguess.game.word.clone().into_iter().collect::<String>()));
                return Task::none()
            }
        }
    }
    Task::none()
}