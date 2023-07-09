mod direction;
mod food;
mod game;
mod gameover;
mod highscore;
mod mainmenu;
mod score;
mod snake;
mod textblock;

use crate::{
    game::Game, game::GameState, gameover::GameOver, highscore::HighScore, mainmenu::MainMenu,
};
use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow},
    system::{Clock, Time},
    window::{ContextSettings, Event, Style},
};
pub const SCALE: f32 = 16.;
pub const WIDTH: f32 = 50.;
pub const HEIGHT: f32 = 38.;

type Handler = fn(Event, &mut game::Game);

fn main() {
    let font = Font::from_file("src/assets/Minimal3x5.ttf").unwrap();
    let mut clock = Clock::start();
    let mut time: Time;
    let mut rw = RenderWindow::new(
        ((SCALE * WIDTH) as u32, (SCALE * HEIGHT) as u32),
        "Boitata",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut game = Game::new();
    let mut handler: Handler = MainMenu::handler;
    rw.set_vertical_sync_enabled(true);

    while rw.is_open() {
        rw.clear(Color::BLACK);

        match game.get_game_state() {
            game::GameState::Running => {
                handler = game::Game::handler;
                game.update_render(&mut rw);
                time = clock.elapsed_time();
                if time.as_seconds() >= game.get_speed() {
                    game.tick();
                    clock.restart();
                    rw.clear(Color::BLACK);
                }
            }
            GameState::GameOver => {
                handler = GameOver::handler;
                GameOver::render(&mut rw, &font, &game);
            }
            GameState::MainMenu => {
                handler = MainMenu::handler;
                MainMenu::render(&mut rw, &font);
            }
            GameState::HighScore => {
                handler = HighScore::handler;
                HighScore::render(&mut rw, &font, &mut game);
            }
            GameState::Quit => {
                rw.close();
            }
        }

        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                _ => handler(ev, &mut game),
            }
        }
    }
}
