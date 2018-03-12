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
    events: Events,
    figure_idx: usize,
    /// Stored events that should be run next are in this vector
    event_stack: Vec<matplotrs_backend::Event>,
    figure_id_count: usize,
}

struct Figure {
    w: Window,
    id: matplotrs_backend::FigureId,
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
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
            events: Events::new(EventSettings::new()),
            figure_idx: 0,
            event_stack: Vec::new(),
            figure_id_count: 0,
        }
    }

    fn new_figure(&mut self, title: &str, size: &(f64, f64)) -> Result<matplotrs_backend::FigureId, Self::Err> {
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
        let id = matplotrs_backend::FigureId(self.figure_id_count);
        self.figures.push(Figure {
            w: window,
            id,
            gl: GlGraphics::new(OPENGL_VERSION),
            rotation: 0.0,
        });
        self.figure_id_count += 1;
        Ok(id)
    }

    fn draw_path(&mut self, fig_id: matplotrs_backend::FigureId, path: &matplotrs_backend::Path) -> Result<(), Self::Err> {
        use graphics::*;
        let fig = self.figure_by_id(fig_id).ok_or("Find figure")?;
        let gl = &mut fig.gl;
        // TODO: Get real values here!
        let view_port = Viewport {
            rect: [0, 0, 1000, 1000],
            draw_size: [1000, 1000],
            window_size: [1000, 1000],
        };
        let (x, y) = (0.0, 0.0);
        gl.draw(view_port, |c, gl| {
            // TODO: clear somewhere else(fig background color, gl);
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            let line_color = path.line_color.map(to_webgl_color).unwrap_or(BLACK);
            let transform = c.transform//.trans(x, y)
                                       .trans(30.0, 30.0);
            clear(WHITE, gl);
            let my_line = [1.0, 1.0, 100.0, 100.0];
            line(BLACK, 10.0, my_line, transform, gl);
            // for &(x, y) in path.points.iter() {
            //     let my_line = Line::new(line_color, 0.5);
            //     line(my_line, transform, &DrawState::default(), gl, gl);
            // }
        });
        Ok(())
    }

    fn draw_text(&mut self, _: matplotrs_backend::FigureId, _: &matplotrs_backend::Text) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_image(&mut self, _: matplotrs_backend::FigureId, _: &matplotrs_backend::Image) -> Result<(), Self::Err> {
        Ok(())
    }

    fn save_to_file(&mut self) -> Result<(), Self::Err> {
        unimplemented!()
    }

    fn next_event(&mut self) -> Option<matplotrs_backend::Event> {
        self.event_stack.pop().or_else(|| {
            let len = self.figures.len();
            if len == 0 {
                // No figure, so nothing to do
                println!("No more event to process in Piston backend!");
                None
            } else {
                if self.figure_idx >= len {
                    self.figure_idx = 0;
                }
                let (event, fig_id) = {
                    let next_figure = &mut self.figures[self.figure_idx];
                    self.figure_idx += 1;
                    (self.events.next(&mut next_figure.w).and_then(convert_events), next_figure.id)
                };
                event.map(|e| {
                    matplotrs_backend::Event { e, fig_id }
                }).or_else(|| self.next_event())
            }
        })
    }
}

impl From<String> for PistonError {
    fn from(err: String) -> Self {
        PistonError::BackEndError(err)
    }
}
impl<'a> From<&'a str> for PistonError {
    fn from(err: &str) -> Self {
        PistonError::BackEndError(err.to_owned())
    }
}

fn to_webgl_color((r, g, b, a): (f64, f64, f64, f64)) -> [f32; 4] {
    [r as f32, g as f32, b as f32, a as f32]
}

fn convert_events(event: Event) -> Option<matplotrs_backend::EventKind> {
    match event {
        Event::Input(input) => match input {
            Input::Button(_args) => None, /* TODO Ignore for now! */
            Input::Move(_motion) => None, /* TODO Ignore for now! */
            Input::Text(_) => None, /* TODO Ignore for now! */
            Input::Resize(_w, _h) => None,
            Input::Focus(_focus) => None,
            Input::Cursor(_cursor) => None, /* TODO Ignore for now! */
            Input::Close(_) => None, /* TODO Ignore for now! */
        },
        Event::Loop(lp) => match lp {
            Loop::Render(_args) => Some(matplotrs_backend::EventKind::Render),
            Loop::AfterRender(_args) => None,
            Loop::Update(args) => Some(matplotrs_backend::EventKind::Update(args.dt)),
            Loop::Idle(_args) => None,
        }
        _ => unimplemented!(),
    }
}

impl PistonBackend {
    fn figure_by_id(&mut self, fig_id: matplotrs_backend::FigureId) -> Option<&mut Figure> {
        for fig in self.figures.iter_mut() {
            if fig.id == fig_id {
                return Some(fig);
            }
        }
        None
    }
}
