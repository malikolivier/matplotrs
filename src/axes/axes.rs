//use std::rc::Weak;

use color::{Color, WHITE};
use figure::Figure;
use artist::Artist;

pub struct Axes<'f> {
    a: AxesAttributes,
    children: Vec<Box<Artist>>,
    /// Include parent figure for reference (if needed)
    fig: &'f Figure,
}

pub struct AxesBuilder<'f> {
    pub a: AxesAttributes,
    pub fig: &'f Figure,
}

pub struct AxesAttributes {
    rect: [f64; 4],
    facecolor: Color,
}

impl<'f> Artist for Axes<'f> {
}

impl<'f> AxesBuilder<'f> {
    fn build(self) -> Axes<'f> {
        Axes { a: self.a, children: Vec::new(), fig: self.fig }
    }

    fn with_rect(mut self, rect: &[f64; 4]) -> Self {
        self.a.rect = rect.clone();
        self
    }

    fn with_facecolor<T: Into<Color>>(mut self, color: T) -> Self {
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
