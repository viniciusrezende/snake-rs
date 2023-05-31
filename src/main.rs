use std::{borrow::Borrow};
use rand::prelude::*;

use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow, Shape, RectangleShape, Transformable
    },
    window::{ContextSettings, Event, Key, Style},
    system::{Clock, Time},
};

pub const SCALE: f32 = 20.;
#[derive(Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Food {
    x: f32,
    y: f32,
}
impl Food {
    pub fn new(starting_x:f32 , starting_y:f32) -> Food {
        Food {
            x: starting_x,
            y: starting_y,
        }
    }
    pub fn regenerate(&mut self) {
        let mut rng = rand::thread_rng();
        let posx:f32 = rng.gen();
        let posy:f32 = rng.gen();
        self.x = (40. * posx).floor();
        self.y = (30. * posy).floor();
        println!("{} {}", self.x, self.y);
    }
}
pub struct SnakeBody {
    x: f32,
    y: f32,
}
pub struct Snake {
    body: Vec<SnakeBody>,
    direction: Direction,
    color: Color,
    speed: f32,
    grow: bool,
}
impl Snake {
    pub fn new(starting_x:f32 , starting_y:f32) -> Snake {
        let mut snk = Snake {
            body: vec![SnakeBody { x: starting_x, y: starting_y }],
            direction: Direction::Right,
            color: Color::GREEN,
            speed: 1.,
            grow: false,
        };
        snk.body.push(SnakeBody { x: starting_x-1., y: starting_y });
        snk.body.push(SnakeBody { x: starting_x-2., y: starting_y });
        return snk;
    }

    fn move_forward(&mut self) {
        let mut head:SnakeBody = SnakeBody { x: self.body.first().unwrap().x, y: self.body.first().unwrap().y };
        match self.direction{
            Direction::Up=>head.y-=1.,
            Direction::Down=>head.y+=1.,
            Direction::Right=>head.x+=1.,
            Direction::Left=>head.x-=1.,
            _=>(),
        }
        if ! self.grow {
            self.body.pop();
        }
        self.grow = false;
        println!("{} {} {} {}", self.speed, self.body.len(), head.x, head.y);
        self.body.insert(0, head);
        // check body colision and wall colision
    }
    fn try_to_eat(&mut self, food:&Food) {
        let mut head:SnakeBody = SnakeBody { x: self.body.first().unwrap().x, y: self.body.first().unwrap().y };
        if food.x == head.x && food.y == head.y {
            self.grow=true;
            self.speed*=0.8;
        }
    }
}

fn main() {
    let mut clock = Clock::start();
    let mut time:Time;
    let mut rw = RenderWindow::new(
        (800, 600),
        "Snake Game",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut snake = Snake::new(20.,20.);
    let mut food:Food = Food { x:-1., y:-1. };
    food.regenerate();
    rw.set_vertical_sync_enabled(true);
    
    while rw.is_open() {
        rw.clear(Color::BLACK);
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::KeyReleased { code, .. } => { 
                     match code {
                        Key::Up=> {
                            if snake.direction != Direction::Down {
                                snake.direction = Direction::Up;
                            }
                        },
                        Key::Down=> {
                            if snake.direction != Direction::Up {
                                snake.direction = Direction::Down;
                            }
                        },
                        Key::Left=> {
                            if snake.direction != Direction::Right {
                                snake.direction = Direction::Left;
                            }
                        },
                        Key::Right=> {
                            if snake.direction != Direction::Left {
                                snake.direction = Direction::Right;
                            }
                        },
                        _=>(),
                     }                    
                },
                _ => {}
            }
        }
        // render
        let mut rect = sfml::graphics::RectangleShape::new();
        rect.set_position((food.x*SCALE,food.y*SCALE));
        rect.set_size((SCALE,SCALE));
        rect.set_fill_color(Color::RED);
        rw.draw(&rect);

        for part in &snake.body  {
            let mut rect = sfml::graphics::RectangleShape::new();
            rect.set_position((part.x*SCALE,part.y*SCALE));
            rect.set_size((SCALE,SCALE));
            rect.set_fill_color(snake.color);
            rw.draw(&rect);
        }

        rw.display();
        time = clock.elapsed_time();
        if time.as_seconds() >= snake.speed {
            snake.try_to_eat(&food);
            if snake.grow {
                food.regenerate();
            }
            snake.move_forward();
            clock.restart();
            rw.clear(Color::BLACK);
        }
    }
}
