use matplotrs_backend;

use color::{Color, BLACK};
use artist::Artist;

pub struct Line {
    l: LineAttributes,
}

pub struct LineBuilder {
    l: LineAttributes,
}

pub struct LineAttributes {
    start: (f64, f64),
    end: (f64, f64),
    edgecolor: Color,
}

impl Line {
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Line {
        LineBuilder::new(start, end).build()
    }
}

impl LineBuilder {
    pub fn new(start: (f64, f64), end: (f64, f64)) -> LineBuilder {
        let mut l: LineAttributes = Default::default();
        l.start = start;
        l.end = end;
        LineBuilder { l }
    }

    pub fn with_edgecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.l.edgecolor = color.into();
        self
    }

    pub fn build(self) -> Line {
        Line { l: self.l }
    }
}

impl Artist for Line {
    fn paths(&self) -> Vec<matplotrs_backend::Path> {
        let Color(r, g, b, a) = self.l.edgecolor;
        vec![
            matplotrs_backend::Path {
                points: vec![self.l.start, self.l.end],
                closed: false,
                line_color: Some((r, g, b, a)),
                fill_color: None,
            },
        ]
    }
}

impl Default for LineAttributes {
    fn default() -> Self {
        Self {
            start: (0.0, 0.0),
            end: (0.0, 0.0),
            edgecolor: BLACK,
        }
    }
}
