use sfml::{
    graphics::{
        RenderWindow, Font
    },
    window::{Key},
};

use crate::textblock::TextBlock;
use crate::game::Game;
use crate::game::GameState;
pub struct GameOver {
}

impl GameOver {
    pub fn render( rw: &mut RenderWindow, font: &Font, game:&Game) {
        let messages = [
            String::from("Game Over"),
            format!( "Score {}", game.get_score() ),
            String::from("Press Space to restart")
        ];
        TextBlock::render_center(rw, font, &messages);
        rw.display();

    }
    pub fn handler( code: Key, game: &mut Game) {
        match code {
            Key::Escape => game.set_game_state(GameState::MainMenu),
            Key::Space => { game.reset(); game.set_game_state(GameState::Running); },
            _ => {},
        }
    }
}