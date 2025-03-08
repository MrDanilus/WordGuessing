use iced::{window::Settings, Size};

use ui::WGuess;
mod ui;
mod back;

fn main() -> iced::Result {
    iced::application("Word Guessing", back::update::func, ui::view::func)
        .centered()
        .theme(WGuess::theme)
        .window(Settings{
            size: Size::new(600.0, 700.0),
            ..Default::default()
        })
        .subscription(WGuess::subscription)
        .run()
}