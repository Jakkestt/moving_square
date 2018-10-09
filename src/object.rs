use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL };


pub struct Object {
    gl: GlGraphics,
    x: f64,
    y: f64,
}


#[allow(dead_code)]
impl Object {
    pub fn new() -> Object {
        let opengl = OpenGL::V3_2;
        Object {gl: GlGraphics::new(opengl), x : 0.0, y : 0.0}
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    pub fn colx(&mut self, x: f64) {
        self.x = x;
    }

    pub fn render<G>(&self, gl: &mut G, view: math::Matrix2d) where G: Graphics {
        let square = rectangle::square(0.0, 0.0, 50.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        rectangle(red, square, view.trans(self.x, self.y).trans(-25.0, -25.0), gl);
    }
}
