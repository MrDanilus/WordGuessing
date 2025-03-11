use std::collections::HashSet;

pub fn load_words() -> HashSet<String> {
    let words = include_str!("words.txt");

    let words: HashSet<String> = words.lines()
        .map(|line| line.to_string()) // Убираем ошибки
        .collect();

    words
}

pub fn random_word() -> String{
    let words = load_words();
    let index = rand::random_range(0..words.len()-1);
    words.iter().nth(index).unwrap().clone()
}

pub fn check_word(word: &String) -> bool{
    let words = load_words();
    words.contains(word)
}