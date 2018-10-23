use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use std::path::Path;

pub struct Object {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    sprite: Option<Texture>
}


#[allow(dead_code)]
impl Object {
    pub fn new() -> Object {
        let opengl = OpenGL::V3_2;
        Object {gl: GlGraphics::new(opengl), x : 0.0, y : 0.0, sprite: None}
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub fn render(&self, gl: &mut GlGraphics, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 50.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        match self.sprite {
            None => {
                rectangle(red, square, view.trans(self.x, self.y).trans(-25.0, -25.0), gl);
            }
            Some(ref sprite) => {
                image(sprite, view.trans(self.x, self.y).trans(-25.0, -25.0), gl);
            }
        }
    }
    pub fn set_sprite(&mut self, sprite: Texture) {
        self.sprite = Some(sprite);
    }
}
