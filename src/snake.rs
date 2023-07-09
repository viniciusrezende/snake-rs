use crate::direction::Direction;
use crate::food::Food;
use sfml::graphics::Color;

use crate::HEIGHT;
use crate::WIDTH;
pub struct SnakeBody {
    x: f32,
    y: f32,
}
impl SnakeBody {
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
}
pub struct Snake {
    body: Vec<SnakeBody>,
    next_direction: Direction,
    current_direction: Direction,
    color: Color,
    speed: f32,
    grow: bool,
}
impl Snake {
    pub fn new(starting_x: f32, starting_y: f32) -> Snake {
        let mut snk = Snake {
            body: vec![SnakeBody {
                x: starting_x,
                y: starting_y,
            }],
            current_direction: Direction::Right,
            next_direction: Direction::Right,
            color: Color::GREEN,
            speed: 0.5,
            grow: false,
        };
        snk.body.push(SnakeBody {
            x: starting_x - 1.,
            y: starting_y,
        });
        snk.body.push(SnakeBody {
            x: starting_x - 2.,
            y: starting_y,
        });
        snk
    }

    pub fn move_forward(&mut self) {
        let mut head: SnakeBody = SnakeBody {
            x: self.body.first().unwrap().x,
            y: self.body.first().unwrap().y,
        };
        self.current_direction = self.next_direction;
        match self.current_direction {
            Direction::Up => head.y -= 1.,
            Direction::Down => head.y += 1.,
            Direction::Right => head.x += 1.,
            Direction::Left => head.x -= 1.,
        }
        if !self.grow {
            self.body.pop();
        }
        self.grow = false;
        self.body.insert(0, head);
        // check body colision and wall colision
    }
    pub fn check_colision(&self) -> bool {
        let head: SnakeBody = SnakeBody {
            x: self.body.first().unwrap().x,
            y: self.body.first().unwrap().y,
        };
        if head.x <= 0. || head.x >= WIDTH - 1_f32 || head.y <= 0. || head.y >= HEIGHT - 1_f32
        {
            return true;
        }
        for part in self.body.iter().skip(1) {
            if head.x == part.x && head.y == part.y {
                return true;
            }
        }
        false
    }
    pub fn try_to_eat(&mut self, food: &Food) {
        let head: SnakeBody = SnakeBody {
            x: self.body.first().unwrap().x,
            y: self.body.first().unwrap().y,
        };
        if food.get_x() == head.x && food.get_y() == head.y {
            self.grow = true;
            if self.speed > 0.1 {
                self.speed -= 0.05;
            }
        }
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.next_direction = direction;
    }
    pub fn get_direction(&self) -> Direction {
        self.current_direction
    }
    pub fn get_grow(&self) -> bool {
        self.grow
    }
    pub fn get_body(&self) -> &Vec<SnakeBody> {
        &self.body
    }
}
