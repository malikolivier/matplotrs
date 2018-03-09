use matplotrs_backend;

use color::{Color, WHITE};
use artist::Artist;

pub struct Axes {
    a: AxesAttributes,
    children: Vec<Box<Artist>>,
}

pub struct AxesBuilder {
    pub a: AxesAttributes,
}

pub struct AxesAttributes {
    rect: [f64; 4],
    facecolor: Color,
}

impl Artist for Axes {
    fn path(&self) -> matplotrs_backend::Path {
        let [x, y, dx, dy] = self.a.rect;
        let Color(r, g, b, a) = self.a.facecolor;
        let points = vec![(x, y), (x + dx, y), (x + dx, y + dy), (x, y + dy)];
        matplotrs_backend::Path { points, closed: true, line_color: None, fill_color: Some((r, g, b, a)) }
    }
}

impl AxesBuilder {
    pub fn new() -> AxesBuilder {
        AxesBuilder { a: Default::default() }
    }

    pub fn build(self) -> Axes {
        Axes { a: self.a, children: Vec::new() }
    }

    pub fn with_rect(mut self, rect: &[f64; 4]) -> Self {
        self.a.rect = rect.clone();
        self
    }

    pub fn with_facecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.a.facecolor = color.into();
        self
    }
}

impl Default for AxesAttributes {
    fn default() -> Self {
        AxesAttributes {
            rect: [0.2, 0.2, 0.6, 0.6],
            facecolor: WHITE,
        }
    }
}
