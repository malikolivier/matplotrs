use matplotrs_backend;

use color::{Color, BLACK};
use artist::Artist;

pub struct LineCollection {
    l: LineCollectionAttributes,
}

pub struct LineCollectionBuilder {
    l: LineCollectionAttributes,
}

pub struct LineCollectionAttributes {
    lines: Vec<(f64, f64)>,
    edgecolor: Color,
}

impl LineCollection {
    pub fn new() -> Self {
        Self {
            l: Default::default(),
        }
    }

    pub fn add_vertex(&mut self, vertex: (f64, f64)) {
        self.l.lines.push(vertex);
    }

    pub fn add_vertices<I>(&mut self, vertex: I)
        where I: IntoIterator<Item = (f64, f64)>
    {
        self.l.lines.extend(vertex);
    }
}

impl LineCollectionBuilder {
    pub fn new() -> Self {
        Self {
            l: Default::default(),
        }
    }

    pub fn with_edgecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.l.edgecolor = color.into();
        self
    }

    pub fn with_vertices<I>(mut self, vertex: I) -> Self
        where I: IntoIterator<Item = (f64, f64)>
    {
        self.l.lines.extend(vertex);
        self
    }

    pub fn build(self) -> LineCollection {
        LineCollection {
            l: self.l,
        }
    }
}


impl Default for LineCollectionAttributes {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            edgecolor: BLACK,
        }
    }
}

impl Artist for LineCollection {
    fn paths(&self) -> Vec<matplotrs_backend::Path> {
        let Color(r, g, b, a) = self.l.edgecolor;
        vec![
            matplotrs_backend::Path {
                points: self.l.lines.clone(),
                closed: false,
                line_color: Some((r, g, b, a)),
                fill_color: None,
            },
        ]
    }
}
