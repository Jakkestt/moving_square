use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use rand::prelude::*;

pub struct Tree {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    sprite: Option<Texture>
}

#[allow(dead_code)]
impl Tree {
    pub fn new() -> Tree {
        let opengl = OpenGL::V3_2;
        let mut rng = thread_rng();
        Tree {gl: GlGraphics::new(opengl), x: rng.gen_range(-600.0, 600.0), y: rng.gen_range(-300.0, 300.0), sprite: None}
    }
    pub fn moar_trees(&self, gl: &mut GlGraphics, view: math::Matrix2d) {
        let tree = rectangle::square(0.0, 0.0, 300.0);
        let blue = [0.0, 0.0, 1.0, 1.0];
        match self.sprite {
            None => {
                rectangle(blue, tree, view.trans(self.x, self.y).trans(-150.0, -150.0), gl)
            }
            Some(ref sprite) =>{
                image(sprite, view.trans(self.x, self.y).trans(-150.0, -150.0), gl)
            }
        }
    }
    pub fn set_sprite(&mut self, sprite: Texture) {
        self.sprite = Some(sprite);
    }
}
