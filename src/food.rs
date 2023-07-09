use crate::HEIGHT;
use crate::WIDTH;
use rand::prelude::*;
pub struct Food {
    x: f32,
    y: f32,
}
impl Food {
    pub fn new(starting_x: f32, starting_y: f32) -> Food {
        Food {
            x: starting_x,
            y: starting_y,
        }
    }
    pub fn regenerate(&mut self) {
        let mut rng = rand::thread_rng();
        let posx: f32 = rng.gen();
        let posy: f32 = rng.gen();
        self.x = (1. + ((WIDTH - 2.) * posx)).floor();
        self.y = (1. + ((HEIGHT - 2.) * posy)).floor();
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
}
