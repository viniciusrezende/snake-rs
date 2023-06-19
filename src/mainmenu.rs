use sfml::{
    graphics::{
        RenderWindow, Font
    },
    window::{Key},
};

use crate::textblock::TextBlock;
use crate::game::Game;
use crate::game::GameState;
pub struct MainMenu {
}

impl MainMenu {
    pub fn render( rw: &mut RenderWindow, font: &Font) {
        let messages = [
            String::from("Boitata"),
            String::from(""),
            String::from("1 Start"),
            String::from("2 High Scores"),
            String::from("3 Quit")
        ];
        TextBlock::render_center(rw, font, &messages);
        rw.display();

    }
    pub fn handler( code: Key, game: &mut Game) {
        match code {
            Key::Num1 => { game.reset(); game.set_game_state(GameState::Running); },
            Key::Num2 => game.set_game_state(GameState::HighScore),
            Key::Num3 => game.set_game_state(GameState::Quit),
            _ => {},
        }
    }
}