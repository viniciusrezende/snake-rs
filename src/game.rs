use crate::food::Food;
use crate::snake::Snake;
use crate::direction::Direction;
use crate::SCALE;
use crate::WIDTH;
use crate::HEIGHT;

use sfml::graphics::{RenderWindow,RectangleShape,Shape, Transformable, Color, RenderTarget};
use sfml::window::Key;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum GameState {
    Running,
    GameOver,
    MainMenu,
    HighScore,
    Quit,
}
pub struct Game {
    snake: Snake,
    food: Food,
    state: GameState,
    score: u8
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            snake: Snake::new(20.,20.),
            food: Food::new(-1.,-1.),
            state: GameState::MainMenu,
            score: 0
        };
        
        game.food.regenerate();
        return game;
    }
    pub fn reset(&mut self) {
        self.snake = Snake::new(20.,20.);
        self.food = Food::new(-1.,-1.);
        self.score = 0;
        self.food.regenerate();
    }
    fn render_wall(&mut self, rw: &mut RenderWindow) {
        let mut rect = RectangleShape::new();
        rect.set_position((0.,0.));
        rect.set_size((WIDTH*SCALE,SCALE));
        rect.set_fill_color(Color::BLUE);
        rw.draw(&rect);
        rect.set_position((0.,HEIGHT*SCALE-SCALE));
        rw.draw(&rect);
        rect.set_size((SCALE,HEIGHT*SCALE));
        rect.set_position((0.,0.));
        rw.draw(&rect);
        rect.set_position((WIDTH*SCALE-SCALE,0.));
        rw.draw(&rect);
    }

    fn render_snake(&mut self, rw: &mut RenderWindow) {
        for part in self.snake.get_body()  {
            let mut rect = RectangleShape::new();
            rect.set_position((part.get_x()*SCALE,part.get_y()*SCALE));
            rect.set_size((SCALE,SCALE));
            rect.set_fill_color(self.snake.get_color());
            rw.draw(&rect);
        }
    }

    fn render_food(&mut self, rw: &mut RenderWindow) {
        let mut rect = RectangleShape::new();
        rect.set_position((self.food.get_x()*SCALE,self.food.get_y()*SCALE));
        rect.set_size((SCALE,SCALE));
        rect.set_fill_color(Color::RED);
        rw.draw(&rect);
    }
    pub fn update_render(&mut self, rw: &mut RenderWindow) {
        self.render_wall(rw);
        self.render_food(rw);
        self.render_snake(rw);

        rw.display();
    
    }
    
    pub fn tick(&mut self) {
        self.snake.try_to_eat(&self.food);
        if self.snake.get_grow() {
            self.inc_score();
            self.food.regenerate();
        }
        self.snake.move_forward();
        if self.snake.check_colision() {
            self.set_game_state(GameState::GameOver);
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.snake.get_speed()
    }

    pub fn handler( code: Key, game: &mut crate::game::Game) {
        match code {
            Key::Up=> {
                if game.snake.get_direction() != Direction::Down {
                    game.snake.set_direction(Direction::Up);
                }
            },
            Key::Down=> {
                if game.snake.get_direction() != Direction::Up {
                    game.snake.set_direction(Direction::Down);
                }
            },
            Key::Left=> {
                if game.snake.get_direction() != Direction::Right {
                    game.snake.set_direction(Direction::Left);
                }
            },
            Key::Right=> {
                if game.snake.get_direction() != Direction::Left {
                    game.snake.set_direction(Direction::Right);
                }
            },
            Key::Space=> {
                game.food.regenerate();
            }
            _=>(),
        }
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.state = state;
    }
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    pub fn get_score(&self) -> u8 {
        self.score
    }
    pub fn inc_score(&mut self) {
        self.score+=1;
    }
}