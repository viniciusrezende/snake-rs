mod snake;
mod food;
mod direction;
mod game;

use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow, Font
    },
    window::{ContextSettings, Event, Style},
    system::{Clock, Time},
};

pub const SCALE: f32 = 16.;
pub const WIDTH: f32 = 50.;
pub const HEIGHT: f32 = 38.;


fn main() {
    let font = Font::from_file("src/assets/Minimal3x5.ttf").unwrap();
    let mut clock = Clock::start();
    let mut time:Time;
    let mut rw = RenderWindow::new(
        ( ( SCALE*WIDTH ) as u32, ( SCALE*HEIGHT ) as u32),
        "Snake Game",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut game = game::Game::new();
    rw.set_vertical_sync_enabled(true);
    
    while rw.is_open() {
        rw.clear(Color::BLACK);

        match game.get_game_state() {
            game::GameState::Running => {
                while let Some(ev) = rw.poll_event() {
                    match ev {
                        Event::Closed => rw.close(),
                        Event::KeyReleased { code, .. } => { 
                            game.set_snake_direction_based_on_keypress(code);
                        },
                        _ => {}
                    }
                }
                // render
                game.update_render(&mut rw);
                time = clock.elapsed_time();
                if time.as_seconds() >= game.get_speed() {
                    game.tick();
                    clock.restart();
                    rw.clear(Color::BLACK);
                }
            },
            game::GameState::GameOver => {
                while let Some(ev) = rw.poll_event() {
                    match ev {
                        Event::Closed => rw.close(),
                        Event::KeyReleased { code, .. } => {
                            if code == sfml::window::Key::Space {
                                game = game::Game::new();
                                game.set_game_state(game::GameState::Running);
                            }
                        },
                        _ => {}
                    }
                }
                game.render_game_over(&mut rw, &font);
            }
            game::GameState::MainMenu => {
                while let Some(ev) = rw.poll_event() {
                    match ev {
                        Event::Closed => rw.close(),
                        Event::KeyReleased { code, .. } => {
                            if code == sfml::window::Key::Space {
                                game.set_game_state(game::GameState::Running);
                            }
                        },
                        _ => {}
                    }
                }
                game.render_main_menu(&mut rw, &font);
            }
            game::GameState::HighScore => {
                while let Some(ev) = rw.poll_event() {
                    match ev {
                        Event::Closed => rw.close(),
                        Event::KeyReleased { code, .. } => {
                            if code == sfml::window::Key::Space {
                                game.set_game_state(game::GameState::MainMenu);
                            }
                        },
                        _ => {}
                    }
                }
                game.render_high_score(&mut rw, &font);
            }
        }
    }
}