extern crate matplotrs_backend;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct PistonBackend {
    figures: Vec<Figure>,
}

struct Figure {
    w: Window,
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl Figure {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

#[derive(Debug)]
pub enum PistonError {
    BackEndError(String),
}

// Change this to OpenGL::V2_1 if not working.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

impl matplotrs_backend::Backend for PistonBackend {
    type Err = PistonError;
    fn new() -> Self {
        PistonBackend {
            figures: Vec::new(),
        }
    }

    fn new_figure(&mut self, title: &str, size: &(f64, f64)) -> Result<(), Self::Err> {
        if self.figures.len() > 0 {
            return Err(From::from("Only one figure is currently supported on piston backend! See https://github.com/PistonDevelopers/piston-examples/issues/401".to_owned()))
        }
        let &(x, y) = size;
        let window = WindowSettings::new(
                title,
                [x as u32, y as u32]
            )
            .opengl(OPENGL_VERSION)
            // ↓ Required for this bug https://github.com/PistonDevelopers/piston/issues/1202
            .srgb(false)
            .exit_on_esc(true)
            .build()?;
        self.figures.push(Figure {
            w: window,
            gl: GlGraphics::new(OPENGL_VERSION),
            rotation: 0.0,
        });
        Ok(())
    }

    fn draw_path(&mut self, _: &matplotrs_backend::Path) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_text(&mut self, _: &matplotrs_backend::Text) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_image(&mut self, _: &matplotrs_backend::Image) -> Result<(), Self::Err> {
        Ok(())
    }

    fn show(mut self) -> Result<i32, Self::Err> {
        for figure in self.figures.iter_mut() {
            let mut events = Events::new(EventSettings::new());
            while let Some(e) = events.next(&mut figure.w) {
                if let Some(r) = e.render_args() {
                    figure.render(&r);
                }

                if let Some(u) = e.update_args() {
                    figure.update(&u);
                }
            }
        }
        println!("Bye!");
        Ok(0)
    }
}

impl From<String> for PistonError {
    fn from(err: String) -> Self {
        PistonError::BackEndError(err)
    }
}
