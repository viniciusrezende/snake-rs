use crate::direction::Direction;
use crate::food::Food;

use crate::snake::Snake;
use crate::HEIGHT;
use crate::SCALE;
use crate::WIDTH;

use sfml::graphics::{ RenderTarget, RenderWindow, Transformable, Texture, Sprite, IntRect};
use sfml::window::{Event, Key};
use sfml::system::{Vector2f};
use sfml::audio::{Sound};
use libm::atan2f;

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
    score: u32,
    name: String,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            snake: Snake::new(20., 20.),
            food: Food::new(-1., -1.),
            state: GameState::MainMenu,
            score: 0,
            name: String::from(""),
        };

        game.food.regenerate();
        game
    }
    pub fn reset(&mut self) {
        self.snake = Snake::new(20., 20.);
        self.food = Food::new(-1., -1.);
        self.score = 0;
        self.food.regenerate();
    }
    fn helper_get_origin(angle:f32) -> Vector2f {
        match angle as u32 {
            0 => Vector2f{x:0., y:0.},
            90 => Vector2f{x:0., y:16.},
            180 => Vector2f{x:16., y:16.},
            270 => Vector2f{x:16., y:0.},
            _ => Vector2f{x:0., y:0.}
        }
    }
    fn render_wall(&mut self, rw: &mut RenderWindow, ss: &Texture) {
        let mut sprite = Sprite::with_texture(ss);
        let mut offset:i32;
        let mut rotation: f32;
        sprite.scale(Vector2f{x:SCALE/16., y:SCALE/16.});
        sprite.set_texture_rect(IntRect{left:96, top:0, width:16, height:16});
        for i in 0..(WIDTH as u32) {
            for j in 0..(HEIGHT as u32) {
                rotation=0.;
                offset=96;
                /*
                 * This is not my brightest code. I'm sorry.
                 * It definitely needs a refactor.
                 * 
                 * I couldn't use texture repeat(tiling) because I was using spritesheet.
                 * Possibly would work for single file as texture.
                 */
                if i>0 && i<WIDTH as u32-1 && j>0 && j<HEIGHT as u32-1 {
                    offset=80;
                } else if i==0 && j==0 {
                    offset=112;
                } else if i==0 && j==HEIGHT as u32-1 {
                    offset=112;
                    rotation = 270.;
                } else if i==WIDTH as u32-1 && j==0 {
                    offset=112;
                    rotation = 90.;
                } else if i==WIDTH as u32-1 && j==HEIGHT as u32-1 {
                    offset=112;
                    rotation = 180.;
                } else if i==0 {
                    rotation = 270.;
                } else if i==WIDTH as u32-1 {
                    rotation = 90.;
                } else if j==HEIGHT as u32-1 {
                    rotation = 180.;
                }

                sprite.set_origin(Self::helper_get_origin(rotation));
                sprite.set_rotation(rotation);
                sprite.set_texture_rect(IntRect{left:offset, top:0, width:16, height:16});
                sprite.set_position((i as f32 * SCALE, j as f32 * SCALE));
                rw.draw(&sprite);
            }
        }
    }

    fn render_snake(&mut self, rw: &mut RenderWindow, ss: &Texture) {
        for (i, part) in self.snake.get_body().iter().enumerate() {
            let mut sprite = Sprite::with_texture(&ss);
            let mut angle;
            sprite.set_position((part.get_x() * SCALE, part.get_y() * SCALE));
            sprite.set_texture_rect(IntRect{left:0, top:0, width:16, height:16});
            if i == 0 {
                sprite.set_texture_rect(IntRect{left:16, top:0, width:16, height:16});
                angle = atan2f(
                    part.get_y() - self.snake.get_body().get(1).unwrap().get_y(), 
                    part.get_x() - self.snake.get_body().get(1).unwrap().get_x()
                );
            } else if i == self.snake.get_body().len() - 1 {
                sprite.set_texture_rect(IntRect{left:48, top:0, width:16, height:16});
                angle = atan2f(
                    self.snake.get_body().get(i-1).unwrap().get_y() - part.get_y(), 
                    self.snake.get_body().get(i-1).unwrap().get_x() - part.get_x()
                );
            } else {
                angle = atan2f(
                    self.snake.get_body().get(i-1).unwrap().get_y() - self.snake.get_body().get(i+1).unwrap().get_y(), 
                    self.snake.get_body().get(i-1).unwrap().get_x() - self.snake.get_body().get(i+1).unwrap().get_x()
                );
                sprite.set_texture_rect(IntRect{left:0, top:0, width:16, height:16});
            }
            
            angle = angle.to_degrees();

            if angle % 90. != 0.0 {
                sprite.set_texture_rect(IntRect{left:32, top:0, width:16, height:16});
                angle -= 45.;
                if angle%180. == 0. && self.snake.get_body().get(i-1).unwrap().get_x() != part.get_x() {
                    angle -= 180.;
                } else if (angle-90.)%180. == 0. && part.get_x() != self.snake.get_body().get(i+1).unwrap().get_x() {
                    angle -= 180.;
                }
                
                if angle < 0. {
                    angle += 360.;
                }
            }
            if angle < 0. {
                angle += 360.;
            }
            sprite.set_origin(Self::helper_get_origin(angle));
            sprite.scale(Vector2f{x:SCALE/16., y:SCALE/16.});
            sprite.rotate(angle);
            rw.draw(&sprite);
        }
    }

    fn render_food(&mut self, rw: &mut RenderWindow, ss: &Texture) {
        let mut sprite = Sprite::with_texture(ss);
        sprite.set_position((self.food.get_x() * SCALE, self.food.get_y() * SCALE));
        sprite.set_texture_rect(IntRect{left:64, top:0, width:16, height:16});
        sprite.set_position((self.food.get_x() * SCALE, self.food.get_y() * SCALE));
        sprite.scale(Vector2f{x:SCALE/16., y:SCALE/16.});
        rw.draw(&sprite);
    }
    pub fn update_render(&mut self, rw: &mut RenderWindow, ss: &Texture) {
        self.render_wall(rw, ss);
        self.render_food(rw, ss);
        self.render_snake(rw, ss);

        rw.display();
    }

    pub fn tick(&mut self, sound: &mut Sound) {
        self.snake.try_to_eat(&self.food);
        if self.snake.get_grow() {
            sound.play();
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

    pub fn handler(event: Event, game: &mut Game) {
        match event {
            Event::KeyPressed { code, .. } => match code {
                Key::Up => {
                    if game.snake.get_direction() != Direction::Down {
                        game.snake.set_direction(Direction::Up);
                    }
                }
                Key::Down => {
                    if game.snake.get_direction() != Direction::Up {
                        game.snake.set_direction(Direction::Down);
                    }
                }
                Key::Left => {
                    if game.snake.get_direction() != Direction::Right {
                        game.snake.set_direction(Direction::Left);
                    }
                }
                Key::Right => {
                    if game.snake.get_direction() != Direction::Left {
                        game.snake.set_direction(Direction::Right);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.state = state;
    }
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn inc_score(&mut self) {
        self.score += 1;
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn push_name(&mut self, c: char) {
        self.name.push(c);
    }
    pub fn pop_name(&mut self) {
        self.name.pop();
    }
}
