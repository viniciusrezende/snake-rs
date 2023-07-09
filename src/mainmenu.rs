use sfml::{
    graphics::{Font, RenderWindow},
    window::{Event, Key},
};

use crate::game::Game;
use crate::game::GameState;
use crate::textblock::TextBlock;
pub struct MainMenu {}

impl MainMenu {
    pub fn render(rw: &mut RenderWindow, font: &Font) {
        let messages = [
            String::from("Boitata"),
            String::from(""),
            String::from("1 Start"),
            String::from("2 High Scores"),
            String::from("3 Quit"),
        ];
        TextBlock::render_center(rw, font, &messages);
        rw.display();
    }
    pub fn handler(event: Event, game: &mut Game) {
        match event {
            Event::KeyPressed { code, .. } => match code {
                Key::Num1 => {
                    game.reset();
                    game.set_game_state(GameState::Running);
                }
                Key::Num2 => game.set_game_state(GameState::HighScore),
                Key::Num3 => game.set_game_state(GameState::Quit),
                _ => {}
            },
            _ => {}
        }
    }
}
