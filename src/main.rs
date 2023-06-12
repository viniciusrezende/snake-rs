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

fn main() {
    let mut clock = Clock::start();
    let mut time:Time;
    let mut rw = RenderWindow::new(
        (800, 600),
        "Snake Game",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut game = game::Game::new();
    rw.set_vertical_sync_enabled(true);
    
    while rw.is_open() {
        rw.clear(Color::BLACK);
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
    }
}
