use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow,
    },
    window::{ContextSettings, Event, Style},
};

fn main() {
    let mut rw = RenderWindow::new(
        (800, 600),
        "Hello World",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_vertical_sync_enabled(true);
    rw.clear(Color::RED);
    while rw.is_open() {
        rw.clear(Color::RED);
        rw.display();
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                _ => {}
            }
        }
    }
}
