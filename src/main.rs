extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate rand;
extern crate sprite;

mod object;
mod tree;
use object::Object;
use tree::Tree;

use piston::window::WindowSettings;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
use std::path::Path;
use std::rc::Rc;
use std::borrow::BorrowMut;

pub struct Cube {
    gl: GlGraphics,
    player: Object,
    trees: Tree,
    height: f64,
    width: f64,
    size: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}

impl Cube {
    fn on_load(&mut self, w: &PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let texture = assets.join("rust.png");
        let texture = Texture::from_path(
                &mut *w.factory.borrow_mut(),
                &texture,
                Flip::None,
                &TextureSettings::new())
                .unwrap();
        self.player.set_sprite(texture);
    }
    fn on_draw(&mut self, args: &RenderArgs) {
        let fuck_this = &self.player;
        let fuck_trees = &self.trees;
        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        let textx = self.player.x.to_string();
        let texty = self.player.y.to_string();

        self.gl.draw(args.viewport(), |c, gl| {
            let center = c.transform.trans((args.width / 2) as f64, (args.height / 2) as f64);
            let randomplace = c.transform.trans(300.0, 300.0);
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
            fuck_trees.moar_trees(gl, center);
            fuck_this.render(gl, center);
        });
    }
    fn update(&mut self, upd: &UpdateArgs) {
        let widthcol = (self.width / 2.0) as f64;
        let heightcol = (self.height / 2.0) as f64;
        let rad = (self.size / 2.0) as f64;
        if self.player.x <= -widthcol + rad {
            self.left_d = false;
        }
        if self.player.x >= widthcol - rad {
            self.right_d = false;
        }
        if self.player.y <= -heightcol + rad {
            self.up_d = false;
        }
        if self.player.y >= heightcol - rad {
            self.down_d = false;
        }
        if self.player.x <= self.trees.x {
            self.left_d = false;
        }
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

    let mut window: PistonWindow = WindowSettings::new("Welcome to the bonezone", (1280, 720))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    
    let mut cube = Cube {
        gl: GlGraphics::new(opengl),
        player : Object::new(),
        trees : Tree::new(),
        height: 720.0,
        width: 1280.0,
        size: 50.0,
        up_d: false,
        down_d: false,
        left_d: false,
        right_d: false
    };
    cube.on_load(&window);
    while let Some(e) = window.next() {
        use graphics::*;
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
