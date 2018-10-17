use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Tree {
    gl: GlGraphics,
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Tree {
    pub fn new() -> Tree {
        let opengl = OpenGL::V3_2;
        Tree {gl: GlGraphics::new(opengl), x: 0.0, y: 0.0}
    }
    pub fn moar_trees<G>(&self, gl: &mut G, view: math::Matrix2d) where G: Graphics {
        let tree = rectangle::square(0.0, 0.0, 300.0);
        let blue = [0.0, 0.0, 1.0, 1.0];
        rectangle(blue, tree, view.trans(self.x, self.y).trans(150.0, 150.0), gl)
    }
}
