use sfml::{
    graphics::{
        RenderWindow, Font
    },
    window::{Key},
};

use crate::textblock::TextBlock;
use crate::game::Game;
use crate::game::GameState;
pub struct HighScore {
}

impl HighScore {
    pub fn render( rw: &mut RenderWindow, font: &Font) {
        let messages = [
            String::from("Highscore"),
            String::from(""),
            String::from("axasxa 1231"),
            String::from("axasxa 1231"),
            String::from("axasxa 1231"),
            String::from("axasxa 1231"),
            String::from("axasxa 1231"),
            String::from(""),
            String::from("Esc to return")
        ];
        TextBlock::render_center(rw, font, &messages);
        rw.display();

    }
    pub fn handler( code: Key, game: &mut Game) {
        match code {
            Key::Escape => game.set_game_state(GameState::MainMenu),
            _ => println!("Other"),
        }
    }
}