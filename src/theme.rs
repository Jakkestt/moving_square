use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use std::path::Path;

pub struct Theme {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    sprite: Option<Texture>
}

#[allow(dead_code)]
impl Theme {
    pub fn new() -> Theme {
        let opengl = OpenGL::V3_2;
        Theme {gl: GlGraphics::new(opengl), x : 0.0, y : 0.0, sprite: None}
    }
    pub fn rendertheme(&self, gl: &mut GlGraphics, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 0.0);
        match self.sprite {
            None => {
                rectangle([0.0, 0.0, 0.0, 0.0], square, view.trans(self.x, self.y).trans(-25.0, -25.0), gl)
            }
            Some(ref sprite) => {
                image(sprite, view.trans(self.x, self.y).trans(-1280.0 / 2.0, -720.0 / 2.0), gl)
            }
        }
    }
    pub fn set_sprite(&mut self, sprite: Texture) {
        self.sprite = Some(sprite);
    }

}
