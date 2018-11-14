use piston_window::*;
use opengl_graphics::{ GlGraphics, Texture };

pub struct Lawn {
    pub x: f64,
    pub y: f64,
    sprite: Option<Texture>
}

#[allow(dead_code)]
impl Lawn {
    pub fn new(i: usize) -> Lawn {
        Lawn {x : (i as f64) * 100.0, y : (i as f64) * 100.0, sprite: None}
    }
    pub fn renderterrain(&self, gl: &mut GlGraphics, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 64.0);
        match self.sprite {
            None => {
                rectangle([0.0, 0.0, 1.0, 1.0], square, view.trans(self.x, self.y).trans(-32.0, -32.0), gl)
            }
            Some(ref sprite) => {
                image(sprite, view.trans(self.x, self.y).trans(-32.0, -32.0), gl)
            }
        }
    }
    pub fn set_sprite(&mut self, sprite: Texture) {
        self.sprite = Some(sprite);
    }

}
