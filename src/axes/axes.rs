use backend::Backend;
use matplotrs_backend::Backend as BackendTrait;
use matplotrs_backend;

use color::Color;
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
    facecolor: Option<Color>,
    edgecolor: Option<Color>,
}

impl Artist for Axes {
    fn paths(&self) -> Vec<matplotrs_backend::Path> {
        let [x, y, dx, dy] = self.a.rect;
        let points = vec![(x, y), (x + dx, y), (x + dx, y + dy), (x, y + dy)];
        vec![
            matplotrs_backend::Path {
                points,
                closed: true,
                line_color: self.a.edgecolor.map(|Color(r, g, b, a)| (r, g, b, a)),
                fill_color: self.a.facecolor.map(|Color(r, g, b, a)| (r, g, b, a)),
            },
        ]
    }

    fn render_children(&self, be: &mut Backend) -> Result<(), <Backend as BackendTrait>::Err> {
        for artist in self.children.iter() {
            let paths = artist.paths();
            for path in paths {
                // Need to transform path's position for it to be used here!
                let path = self.transform_path(path);
                be.draw_path(&path)?;
            }
            artist.render_children(be)?;
        }
        Ok(())
    }
}

impl AxesBuilder {
    pub fn new() -> AxesBuilder {
        AxesBuilder { a: Default::default() }
    }

    pub fn build(self) -> Axes {
        Axes {
            a: self.a,
            children: Vec::new(),
        }
    }

    pub fn with_rect(mut self, rect: &[f64; 4]) -> Self {
        self.a.rect = rect.clone();
        self
    }

    pub fn with_facecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.a.facecolor = Some(color.into());
        self
    }

    pub fn with_edgecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.a.edgecolor = Some(color.into());
        self
    }
}

impl Default for AxesAttributes {
    fn default() -> Self {
        AxesAttributes {
            rect: [-0.6, -0.6, 1.2, 1.2],
            facecolor: None,
            edgecolor: None,
        }
    }
}

impl Axes {
    pub fn add_child<C: Artist + 'static>(&mut self, child: C) {
        self.children.push(Box::new(child));
    }

    /// Transform child's path to parent's coordinate system
    fn transform_path(&self, mut path: matplotrs_backend::Path) -> matplotrs_backend::Path {
        let [x, y, dx, dy] = self.a.rect;
        for point in path.points.iter_mut() {
            let (px, py) = *point;
            *point = (x + dx / 2.0 * (1.0 + px), y + dy / 2.0 * (1.0 + py));
        }
        path
    }
}
