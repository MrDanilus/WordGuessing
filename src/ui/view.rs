use iced::Element;

use crate::ui::WGuess;
use super::{
    pages::{game, menu}, 
    Message, Page
};

pub fn func(wguess: &WGuess) -> Element<Message> {
    return match wguess.page{
        Page::Menu => menu::func(wguess),
        Page::Game => game::func(wguess)
    }
}