use sfml::{
    graphics::{Font, RenderWindow},
    window::{Event, Key},
};

use crate::game::Game;
use crate::game::GameState;
use crate::score::{Score, ScoreRecord};
use crate::textblock::TextBlock;
pub struct HighScore {}

impl HighScore {
    pub fn render(rw: &mut RenderWindow, font: &Font, game: &mut Game) {
        let mut score = Score::new();
        score.fetch();

        let mut messages = [
            String::from("Highscore"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("Esc to return"),
        ];
        // Ugly solution but does the job
        for (i, record) in score.records.iter().enumerate() {
            match record {
                Some(r) => {
                    messages[i + 2] = format!("{} {}", r.score, r.name);
                }
                None => {}
            }
        }

        TextBlock::render_center(rw, font, &messages);
        rw.display();
    }
    pub fn handler(event: Event, game: &mut Game) {
        match event {
            Event::KeyPressed { code, .. } => match code {
                Key::Escape => game.set_game_state(GameState::MainMenu),
                _ => {}
            },
            _ => {}
        }
    }
}
