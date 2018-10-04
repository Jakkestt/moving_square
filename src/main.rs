extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Cube {
    gl: GlGraphics,
    x: f64,
    y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}

impl Cube {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let red = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 100.0);
        let (object_x, object_y) = ((self.x) as f64,
                                    (self.y) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let transform = c.transform.trans((args.width / 2) as f64, (args.height / 2) as f64)
                                        .trans(-25.0, -25.0)
                                        .trans(object_x, object_y);

            rectangle(red, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.up_d {
            //self.player.mov(0.0, -150.0 * upd.dt);
            self.y += -500.0 * args.dt;
        }
        if self.down_d {
            //self.player.mov(0.0, 150.0 * upd.dt);
            self.y += 500.0 * args.dt;
        }
        if self.left_d {
            //self.player.mov(-150.0 * upd.dt, 0.0);
            self.x += -500.0 * args.dt;
        }
        if self.right_d {
            //self.player.mov(150.0 * upd.dt, 0.0);
            self.x += 500.0 * args.dt;
        }
    }
    fn on_input(&mut self, button_args: &ButtonArgs) {
        match button_args.state {
            ButtonState::Press => {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Right => self.right_d = true,
                        Key::Left => self.left_d = true,
                        Key::Down => self.down_d = true,
                        Key::Up => self.up_d = true,
                        _ => {}
                    }
                }
            }
            ButtonState::Release => {

                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Right => self.right_d = false,
                        Key::Left => self.left_d = false,
                        Key::Down => self.down_d = false,
                        Key::Up => self.up_d = false,
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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut cube = Cube {
        gl: GlGraphics::new(opengl),
        x : 0.0,
        y : 0.0,
        up_d: false,
        down_d: false,
        left_d: false,
        right_d: false
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            cube.update(&u);
        }
        if let Some(r) = e.render_args() {
            cube.render(&r);
        }
        if let Some(i) = e.button_args() {
            cube.on_input(&i);
        }
        window.draw_2d(&e, |c, g| {
            let transform = c.transform.trans(10.0, 100.0);

            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                "Hello world!",
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();
        });
    }
}
