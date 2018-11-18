extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate image;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate rand;
extern crate sprite;
extern crate viewport;

mod object;
mod tree;
mod theme;
use theme::Lawn;
use object::Object;
use tree::Tree;

use piston::window::WindowSettings;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };

pub struct Cube {
    gl: GlGraphics,
    player: Object,
    trees: Vec<Tree>,
    terrain: Vec<Lawn>,
    width: f64,
    height: f64,
    viewx: f64,
    viewy: f64,
    size: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}
impl Cube {
    fn on_draw(&mut self, args: &RenderArgs) {
        let fuck_this = &self.player;
        let fuck_trees = &self.trees;
        let fuck_terrain = &self.terrain;
        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        let textx = self.player.x.to_string();
        let texty = self.player.y.to_string();
        let (w, h) = (self.width, self.height);
        self.gl.draw(args.viewport(), |c, gl| {
            let _view = c.transform.trans(w, h);
            let center = c.transform.trans(w / 2.0, h / 2.0);
            clear([0.0, 1.0, 0.0, 0.0], gl);
            for lawn in fuck_terrain {
                lawn.renderterrain(gl, center);
            }
            for tree in fuck_trees {
                tree.moar_trees(gl, center);
            }
            fuck_this.render(gl, center);
            text::Text::new_color([1.0, 0.0, 0.0, 1.0], 25).draw(&textx,
                                                                     &mut glyph_cache,
                                                                     &DrawState::default(),
                                                                     c.transform
                                                                         .trans(10.0, 25.0),
                                                                     gl).unwrap();
            text::Text::new_color([1.0, 0.0, 0.0, 1.0], 25).draw(&texty,
                                                                     &mut glyph_cache,
                                                                     &DrawState::default(),
                                                                     c.transform
                                                                         .trans(10.0, 50.0),
                                                                     gl).unwrap();
        });
    }
    fn update(&mut self, upd: &UpdateArgs) {
        let _rad = (self.size / 2.0) as f64;
        if self.up_d {
            self.player.mov(0.0, -500.0 * upd.dt);
        }
        if self.down_d {
            self.player.mov(0.0, 500.0 * upd.dt);
        }
        if self.left_d {
            self.player.mov(-500.0 * upd.dt, 0.0);
        }
        if self.right_d {
            self.player.mov(500.0 * upd.dt, 0.0);
        }
        self.width = self.viewx - self.player.x * 2.0;
        self.height = self.viewy - self.player.y * 2.0;
    }
    fn on_input(&mut self, button_args: &ButtonArgs) {
        match button_args.state {
            ButtonState::Press => {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::D => self.right_d = true,
                        Key::A => self.left_d = true,
                        Key::S => self.down_d = true,
                        Key::W => self.up_d = true,
                        _ => {}
                    }
                }
            }
            ButtonState::Release => {

                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::D => self.right_d = false,
                        Key::A => self.left_d = false,
                        Key::S => self.down_d = false,
                        Key::W => self.up_d = false,
                        _ => {}
                    }
                }
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let width = 800;
    let height = 600;
    let mut window: PistonWindow = WindowSettings::new("Welcome to the bonezone", (width, height))
        .fullscreen(false)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
        
    let mut cube = Cube {
        gl: GlGraphics::new(opengl),
        player : Object::new(),
        trees : Vec::new(),
        terrain : Vec::new(),
        width: width as f64,
        height: height as f64,
        viewx: width as f64,
        viewy: height as f64,
        size: 50.0,
        up_d: false,
        down_d: false,
        left_d: false,
        right_d: false
    };
    while let Some(e) = window.next() {
        if let Some(u) = e.update_args() {
            cube.update(&u);
        }
        if let Some(r) = e.render_args() {
            cube.on_draw(&r);
        }
        if let Some(i) = e.button_args() {
            cube.on_input(&i);
        }
    }
}
