use crate::food::Food;
use crate::snake::Snake;
use crate::direction::Direction;
use crate::SCALE;
use sfml::graphics::{RenderWindow,RectangleShape,Shape, Transformable, Color, RenderTarget};
use sfml::window::Key;

pub struct Game {
    snake: Snake,
    food: Food
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            snake: Snake::new(20.,20.),
            food: Food::new(-1.,-1.)
        };
        
        game.food.regenerate();
        return game;
    }

    pub fn update_render(&mut self, rw: &mut RenderWindow) {
        let mut rect = RectangleShape::new();
        rect.set_position((self.food.get_x()*SCALE,self.food.get_y()*SCALE));
        rect.set_size((SCALE,SCALE));
        rect.set_fill_color(Color::RED);
        rw.draw(&rect);
        
        for part in &self.snake.body  {
            let mut rect = RectangleShape::new();
            rect.set_position((part.get_x()*SCALE,part.get_y()*SCALE));
            rect.set_size((SCALE,SCALE));
            rect.set_fill_color(self.snake.get_color());
            rw.draw(&rect);
        }

        rw.display();
    
    }
    
    pub fn tick(&mut self) {
        self.snake.try_to_eat(&self.food);
        if self.snake.grow {
            self.food.regenerate();
        }
        self.snake.move_forward();
    }

    pub fn get_speed(&self) -> f32 {
        self.snake.get_speed()
    }

    pub fn set_snake_direction_based_on_keypress(&mut self, code: Key ) {
        match code {
            Key::Up=> {
                if self.snake.get_direction() != Direction::Down {
                    self.snake.set_direction(Direction::Up);
                }
            },
            Key::Down=> {
                if self.snake.get_direction() != Direction::Up {
                    self.snake.set_direction(Direction::Down);
                }
            },
            Key::Left=> {
                if self.snake.get_direction() != Direction::Right {
                    self.snake.set_direction(Direction::Left);
                }
            },
            Key::Right=> {
                if self.snake.get_direction() != Direction::Left {
                    self.snake.set_direction(Direction::Right);
                }
            },
            _=>(),
        }
    }
}