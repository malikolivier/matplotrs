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
use graphics::Viewport;

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
    gl: GlGraphics, // OpenGL drawing backend
}

#[derive(Debug)]
pub enum PistonError {
    BackEndError(String),
}

// Change this to OpenGL::V2_1 if not working.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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

    fn new_figure(&mut self, figure: &matplotrs_backend::FigureRepr) -> Result<matplotrs_backend::FigureId, Self::Err> {
        if self.figures.len() > 0 {
            return Err(From::from("Only one figure is currently supported on piston backend! See https://github.com/PistonDevelopers/piston-examples/issues/401".to_owned()))
        }
        let (x, y) = figure.size;
        let window = WindowSettings::new(
                figure.title.clone(),
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
        });
        self.figure_id_count += 1;
        Ok(id)
    }

    /// Clear figure: Set background color and window name (TODO)
    fn clear_figure(&mut self, fig_id: matplotrs_backend::FigureId, figure: &matplotrs_backend::FigureRepr) -> Result<(), Self::Err> {
        let fig = self.figure_by_id(fig_id).ok_or("Find figure")?;
        let gl = &mut fig.gl;
        let color = to_webgl_color(figure.facecolor);
        gl.draw(to_webgl_viewport((1.0, 1.0)), |_, gl| {
            use graphics::clear;
            clear(color, gl);
        });
        Ok(())
    }

    fn draw_path(&mut self, fig_id: matplotrs_backend::FigureId, path: &matplotrs_backend::Path) -> Result<(), Self::Err> {
        use graphics::*;
        let fig = self.figure_by_id(fig_id).ok_or("Find figure")?;
        let gl = &mut fig.gl;
        // TODO: Get real values here!
        let (fig_width, fig_height) = (1000.0, 1000.0);
        let view_port = to_webgl_viewport((fig_width, fig_height));
        let (x, y) = (fig_width / 2.0, fig_height / 2.0);
        gl.draw(view_port, |c, gl| {
            let line_color = path.line_color.map(to_webgl_color).unwrap_or(BLACK);
            // This transformation puts origin at the center of the viewport and
            // scale the axes so that all values between coordinates -1 and 1
            // are the edge of the screen.
            let transform = c.transform.trans(x, y).scale(x, y);
            let p1_iter = path.points.iter();
            let p2_iter = path.points.iter().skip(1);
            for (p1, p2) in p1_iter.zip(p2_iter) {
                line(line_color, 0.002, [p1.0, p1.1, p2.0, p2.1], transform, gl);
            }
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

fn to_webgl_viewport((width_px, height_px): (f64, f64)) -> Viewport {
    Viewport {
        rect: [0, 0, width_px as i32, height_px as i32],
        draw_size: [width_px as u32, height_px as u32],
        window_size: [width_px as u32, height_px as u32],
    }
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
