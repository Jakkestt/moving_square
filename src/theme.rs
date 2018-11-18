use piston_window::{TextureSettings, Transformed, image, math};
use opengl_graphics::{ GlGraphics, Texture };
use std::path::Path;

pub struct Lawn {
    pub x: f64,
    pub y: f64,
    pub sprite: Texture
}

#[allow(dead_code)]
impl Lawn {
    pub fn new(i:i32, j:i32) -> Lawn {
        let background = Texture::from_path(
                    &Path::new("./assets/background.png"),
                    &TextureSettings::new())
                    .unwrap();
        Lawn {x : (j as f64) * 64.0, y : (i as f64) * -64.0, sprite: background}
    }
    pub fn renderterrain(&self, gl: &mut GlGraphics, view: math::Matrix2d) {
        image(&self.sprite, view.trans(self.x, self.y).trans(-512.0, 512.0), gl)
    }
}
