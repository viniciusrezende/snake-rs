use sfml::graphics::{RenderWindow,Font,Text,Transformable, RenderTarget};

use crate::SCALE;
use crate::WIDTH;
use crate::HEIGHT;

pub struct TextBlock {
}

impl TextBlock {
    const MARGIN: f32 = SCALE*3.;
    pub fn render_center( rw: &mut RenderWindow, font: &Font, messages: &[String]) {
        for (i, message) in messages.iter().enumerate() {
            let mut text = Text::new(message, font, Self::MARGIN as u32);
            text.set_position(
                (
                    WIDTH*SCALE/2.-text.local_bounds().width/2.,
                    HEIGHT*SCALE/2. + ( (i as f32) * Self::MARGIN ) - ( messages.len() as f32 * 0.5 * Self::MARGIN )
                )
            );
            rw.draw(&text);
        }       
    }
}