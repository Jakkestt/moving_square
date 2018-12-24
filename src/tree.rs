use piston_window::{TextureSettings, Transformed, image, math};
use opengl_graphics::{ GlGraphics, Texture };
use rand::prelude::{Rng, thread_rng};
use std::path::Path;

pub struct Tree {
    pub x: f64,
    pub y: f64,
    sprite: Texture
}

#[allow(dead_code)]
impl Tree {
    pub fn new(i:i32) -> Tree {
        let mut rng = thread_rng();
        let treeimg = Texture::from_path(
                        &Path::new("./assets/Tree.png"),
                        &TextureSettings::new())
                        .unwrap();
        Tree {x: (i as f64) * rng.gen_range(-600.0, 600.0), y: (i as f64) * rng.gen_range(-300.0, 300.0), sprite: treeimg}
    }
    pub fn moar_trees(&self, gl: &mut GlGraphics, view: math::Matrix2d) {
        image(&self.sprite, view.trans(self.x, self.y).trans(-32.0, -32.0), gl)
    }
}
