extern crate matplotrs_backend as mb;

extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate texture;

mod events;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use graphics::Viewport;

pub struct PistonBackend {
    figures: Vec<Figure>,
    events: Events,
    figure_idx: usize,
    /// Stored events that should be run next are in this vector
    event_stack: Vec<mb::Event>,
    figure_id_count: usize,
}

struct Figure {
    w: Window,
    id: mb::FigureId,
    /// OpenGL drawing backend
    gl: GlGraphics,
    glyph_cache: GlyphCache<'static>,
    /// Figure size cached in pixel (w, h)
    cached_size: (f64, f64),
}

#[derive(Debug)]
pub enum PistonError {
    BackEndError(String),
    IOError(std::io::Error),
}

// Change this to OpenGL::V2_1 if not working.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl mb::Backend for PistonBackend {
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

    fn new_figure(
        &mut self,
        figure: &mb::FigureRepr,
    ) -> Result<mb::FigureId, Self::Err> {
        if self.figures.len() > 0 {
            return Err(From::from("Only one figure is currently supported on piston backend! See https://github.com/PistonDevelopers/piston-examples/issues/401".to_owned()));
        }
        let (x, y) = figure.size;
        let window = WindowSettings::new(
                figure.title.clone(),
                [x as u32, y as u32]
            )
            .opengl(OPENGL_VERSION)
            // ↓ Required to work around this bug: https://github.com/PistonDevelopers/piston/issues/1202
            .srgb(false)
            .exit_on_esc(true)
            .build()?;
        let id = mb::FigureId(self.figure_id_count);
        self.figures.push(Figure {
            w: window,
            id,
            gl: GlGraphics::new(OPENGL_VERSION),
            glyph_cache: GlyphCache::new(
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                (),
                TextureSettings::new(),
            )?,
            cached_size: figure.size,
        });
        self.figure_id_count += 1;
        Ok(id)
    }

    /// Clear figure: Set background color and window name (TODO)
    fn clear_figure(
        &mut self,
        fig_id: mb::FigureId,
        figure: &mb::FigureRepr,
    ) -> Result<(), Self::Err> {
        let fig = self.figure_by_id(fig_id).ok_or(FIGURE_NOT_FOUND_ERR)?;
        let gl = &mut fig.gl;
        let color = to_webgl_color(figure.facecolor);
        fig.cached_size = figure.size;
        gl.draw(to_webgl_viewport((1.0, 1.0)), |_, gl| {
            use graphics::clear;
            clear(color, gl);
        });
        Ok(())
    }

    /// Draw path to using OpenGL drawing backend.
    fn draw_path(
        &mut self,
        fig_id: mb::FigureId,
        path: &mb::Path,
    ) -> Result<(), Self::Err> {
        use graphics::*;
        let fig = self.figure_by_id(fig_id).ok_or(FIGURE_NOT_FOUND_ERR)?;
        let gl = &mut fig.gl;
        let (fig_width, fig_height) = fig.cached_size;
        let view_port = to_webgl_viewport((fig_width, fig_height));
        let (x, y) = (fig_width / 2.0, fig_height / 2.0);
        gl.draw(view_port, |c, gl| {
            // This transformation puts origin at the center of the viewport and
            // scale the axes so that all values between coordinates -1 and 1
            // are the edge of the screen.
            let transform = c.transform.trans(x, y).scale(x, y);
            // Do not draw filled polygon if no fill_collr is provided
            path.fill_color.map(to_webgl_color).map(|fill_color| {
                // Transform tuples to 2-elem arrays
                // (TODO: Maybe we should use arrays to begin with to avoid this transformation...)
                let poly: Vec<_> = path.points.iter().map(|&(x, y)| [x, y]).collect();
                polygon(fill_color, poly.as_slice(), transform, gl);
            });
            // Do not draw line if no color is provided
            path.line_color.map(to_webgl_color).map(|line_color| {
                // Draw a collection of lines
                let p1_iter = path.points.iter();
                let p2_iter = path.points.iter().skip(1);
                for (p1, p2) in p1_iter.zip(p2_iter) {
                    line(line_color, 0.002, [p1.0, p1.1, p2.0, p2.1], transform, gl);
                }
                if path.closed && !path.points.is_empty() {
                    // Draw last line to close the path
                    let &(x1, y1) = path.points.last().unwrap();
                    let &(x2, y2) = path.points.first().unwrap();
                    line(line_color, 0.002, [x1, y1, x2, y2], transform, gl);
                }
            });
        });
        Ok(())
    }

    fn draw_text(
        &mut self,
        fig_id: mb::FigureId,
        text_to_draw: &mb::Text,
    ) -> Result<(), Self::Err> {
        let fig = self.figure_by_id(fig_id).ok_or(FIGURE_NOT_FOUND_ERR)?;
        let (fig_width, fig_height) = fig.cached_size;
        let view_port = to_webgl_viewport((fig_width, fig_height));
        let (x, y) = (
            fig_width / 2.0 * (1.0 + text_to_draw.point.0),
            fig_height / 2.0 * (1.0 + text_to_draw.point.1),
        );
        let cache = &mut fig.glyph_cache;
        fig.gl
            .draw(view_port, |c, gl| {
                use graphics::Transformed;
                let transform = c.transform.trans(x, y);
                graphics::text(
                    BLACK,
                    text_to_draw.font_size as u32,
                    text_to_draw.text.as_str(),
                    cache,
                    transform,
                    gl,
                )
            })
            .map_err(|e| e.into())
    }

    fn draw_image(
        &mut self,
        fig_id: mb::FigureId,
        image: &mb::Image,
    ) -> Result<(), Self::Err> {
        let fig = self.figure_by_id(fig_id).ok_or(FIGURE_NOT_FOUND_ERR)?;
        let (fig_width, fig_height) = fig.cached_size;
        let view_port = to_webgl_viewport((fig_width, fig_height));
        let (disp_width, disp_height) = image.size;
        let (disp_x, disp_y) = image.position;
        let (pix_w, pix_h) = (image.width as f64, image.height as f64);
        let scale_x = (disp_width / 2.0 * fig_width) / pix_w;
        let scale_y = (disp_height / 2.0 * fig_height) / pix_h;
        let dx = (1.0 + disp_x) * fig_width / 2.0;
        let dy = (1.0 + disp_y - disp_height) * fig_height / 2.0;
        fig.gl.draw(view_port, |c, gl| {
            use graphics::Transformed;
            let transform = c.transform.trans(dx, dy).scale(scale_x, scale_y);
            let image = to_gl_imagebuffer(image);
            let my_texture = Texture::from_image(&image, &texture::TextureSettings::new());
            graphics::image(&my_texture, transform, gl);
        });
        Ok(())
    }

    fn save_to_file(&mut self) -> Result<(), Self::Err> {
        unimplemented!()
    }

    fn next_event(&mut self) -> Option<mb::Event> {
        self.event_stack.pop().or_else(|| {
            let len = self.figures.len();
            if len == 0 {
                // No figure, so nothing to do
                None
            } else {
                if self.figure_idx >= len {
                    self.figure_idx = 0;
                }
                let (event, fig_id) = {
                    let next_figure = &mut self.figures[self.figure_idx];
                    self.figure_idx += 1;
                    (
                        self.events
                            .next(&mut next_figure.w)
                            .and_then(events::convert_events),
                        next_figure.id,
                    )
                };
                event
                    .map(|e| {
                        if let mb::EventKind::Close = e {
                            // A figure has been closed. We must remove it from the figure list.
                            self.remove_figure(fig_id);
                        }
                        mb::Event { e, fig_id }
                    })
                    .or_else(|| self.next_event())
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

impl From<std::io::Error> for PistonError {
    fn from(err: std::io::Error) -> Self {
        PistonError::IOError(err)
    }
}

fn to_gl_imagebuffer(
    img: &mb::Image,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mut data = Vec::with_capacity(img.width * img.height * 4);
    let mut i: usize = 0;
    for p in img.data.iter() {
        data.push(*p);
        // Add alpha channel
        if (i % 3) == 2 {
            data.push(255);
        }
        i += 1;
    }
    image::ImageBuffer::from_raw(img.width as u32, img.height as u32, data).expect("Convert image")
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

fn convert_events(event: Event) -> Option<mb::EventKind> {
    use mb::EventKind;
    match event {
        Event::Input(input) => match input {
            Input::Button(_args) => None, /* TODO Ignore for now! */
            Input::Move(_motion) => None, /* TODO Ignore for now! */
            Input::Text(_) => None,       /* TODO Ignore for now! */
            Input::Resize(w, h) => Some(EventKind::Resize(w, h)),
            Input::Focus(_focus) => None,
            Input::Cursor(_cursor) => None, /* TODO Ignore for now! */
            Input::Close(_) => None,        /* TODO Ignore for now! */
        },
        Event::Loop(lp) => match lp {
            Loop::Render(_args) => Some(EventKind::Render),
            Loop::AfterRender(_args) => None,
            Loop::Update(args) => Some(EventKind::Update(args.dt)),
            Loop::Idle(_args) => None,
        },
        _ => unimplemented!(),
    }
}

impl PistonBackend {
    fn figure_by_id(&mut self, fig_id: mb::FigureId) -> Option<&mut Figure> {
        for fig in self.figures.iter_mut() {
            if fig.id == fig_id {
                return Some(fig);
            }
        }
        None
    }

    fn gl_context_by_fig_id(
        &mut self,
        fig_id: mb::FigureId,
    ) -> Result<&mut GlGraphics, PistonError> {
        let fig = self.figure_by_id(fig_id).ok_or(FIGURE_NOT_FOUND_ERR)?;
        Ok(&mut fig.gl)
    }

    fn remove_figure(&mut self, fig_id: mb::FigureId) {
        self.figures.retain(|fig| fig.id != fig_id);
    }
}

const FIGURE_NOT_FOUND_ERR: &str = "Did not find figure";
