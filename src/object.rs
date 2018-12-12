use piston_window::{TextureSettings, Transformed, image, math};
use opengl_graphics::{ GlGraphics, Texture };
use std::path::Path;

pub struct Object {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    sprite: Texture
}

#[allow(dead_code)]
impl Object {
    pub fn new() -> Object {
        let p1_sprite = Texture::from_path(
                &Path::new("./assets/fuck.png"),
                &TextureSettings::new()).unwrap();
        Object {x : 0.0, y : 0.0, size : 50.0, sprite: p1_sprite}
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
        image(&self.sprite, view.trans(self.x, self.y).trans(-self.size / 2.0, -self.size / 2.0), gl);
    }
}
