mod snake;
mod food;
mod direction;
mod game;

use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow
    },
    window::{ContextSettings, Event, Style},
    system::{Clock, Time},
};

pub const SCALE: f32 = 20.;
pub const WIDTH: f32 = 40.;
pub const HEIGHT: f32 = 30.;


fn main() {
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
                            }
                        },
                        _ => {}
                    }
                }
                rw.display();
            }
        }
    }
}
