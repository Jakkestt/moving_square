use piston_window::{TextureSettings, Transformed, image, math};
use opengl_graphics::{ GlGraphics, Texture };
use std::path::Path;

pub struct Lawn {
    isbn: i32,
    pub x: f64,
    pub y: f64,
    pub sprite: Texture
}

#[allow(dead_code)]
impl Lawn {
    pub fn new(i:f64, j:f64) -> Lawn {
        let background = Texture::from_path(
                    &Path::new("./assets/background.png"),
                    &TextureSettings::new())
                    .unwrap();
        Lawn {x : i * 256.0, y : j * 256.0, sprite: background, isbn : 0}
    }
    pub fn renderterrain(&self, gl: &mut GlGraphics, view: math::Matrix2d) {
        image(&self.sprite, view.trans(self.x, self.y).trans(-256.0 / 2.0, -256.0 / 2.0), gl)
    }
}
impl PartialEq for Lawn {
    fn eq(&self, other: &Lawn) -> bool {
        self.isbn == other.isbn
    }
}