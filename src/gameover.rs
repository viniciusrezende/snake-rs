use sfml::{
    graphics::{Font, RenderWindow},
    window::{Event, Key},
};

use crate::game::Game;
use crate::game::GameState;
use crate::score::Score;
use crate::textblock::TextBlock;

use std::char;
pub struct GameOver {}

impl GameOver {
    pub fn render(rw: &mut RenderWindow, font: &Font, game: &Game) {
        let messages = [
            String::from("Game Over"),
            format!("Score {}", game.get_score()),
            String::from("Enter your name:"),
            game.get_name().clone(),
        ];
        TextBlock::render_center(rw, font, &messages);
        rw.display();
    }
    pub fn handler(event: Event, game: &mut Game) {
        match event {
            Event::KeyPressed { code, .. } => match code {
                Key::Escape => game.set_game_state(GameState::MainMenu),
                Key::Enter => {
                    let mut score = Score::new();
                    score.add(game.get_score(), game.get_name().clone());
                    score.store();
                    game.reset();
                    game.set_game_state(GameState::MainMenu)
                }
                _ => {}
            },
            Event::TextEntered { unicode, .. } => {
                println!("unicode: {} {}", unicode, unicode as u8);
                match unicode as u8 {
                    8 => {
                        game.pop_name();
                    }
                    48..=90 => {
                        game.push_name(unicode as u8 as char);
                    }
                    97..=122 => {
                        game.push_name(unicode as u8 as char);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
