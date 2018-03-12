use backend::Backend;
use matplotrs_backend::{Backend as BackendTrait, FigureId};
use color::Color;
use artist::Artist;

pub struct Figure {
    pub f: FigureAttributes,
    pub children: Vec<Box<Artist>>,
}

pub struct FigureBuilder {
    pub f: Figure,
}

pub struct FigureAttributes {
    pub figsize: (f64, f64),
    pub dpi: f64,
    pub title: Option<String>,
    pub facecolor: Color,
}

impl FigureBuilder {
    pub fn new() -> Self {
        let figure = Figure {
            f: Default::default(),
            children: Vec::new(),
        };
        FigureBuilder { f: figure }
    }

    pub fn with_figsize<W: Into<f64>, H: Into<f64>>(mut self, width: W, height: H) -> Self {
        self.f.f.figsize = (width.into(), height.into());
        self
    }

    pub fn with_dpi<T: Into<f64>>(mut self, dpi: T) -> Self {
        self.f.f.dpi = dpi.into();
        self
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.f.f.title = Some(title.into());
        self
    }

    pub fn with_facecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.f.f.facecolor = color.into();
        self
    }

    pub fn build(self) -> Figure {
        self.f
    }
}

impl Figure {
    pub fn new() -> Self {
        FigureBuilder::new().build()
    }

    pub fn add_child<C: Artist + 'static>(&mut self, child: C) {
        self.children.push(Box::new(child));
    }

    pub fn title(&self) -> Option<&str> {
        self.f.title.as_ref().map(String::as_str)
    }

    pub fn create(&self, be: &mut Backend) -> Result<FigureId, <Backend as BackendTrait>::Err> {
        let title = self.title().unwrap_or("Figure");
        let size = &self.f.figsize;
        Ok(be.new_figure(title, size)?)
    }

    pub fn render(
        &self,
        be: &mut Backend,
        fig_id: FigureId,
    ) -> Result<(), <Backend as BackendTrait>::Err> {
        be.clear_figure(fig_id)?;
        for artist in self.children.iter() {
            for path in artist.paths() {
                be.draw_path(fig_id, &path)?;
            }
            for text in artist.texts() {
                be.draw_text(fig_id, &text)?;
            }
            for image in artist.images() {
                be.draw_image(fig_id, &image)?;
            }
            // Draw inner objects for axis
            artist.render_children(fig_id, be)?;
        }
        Ok(())
    }
}

impl Default for FigureAttributes {
    fn default() -> Self {
        Self {
            figsize: (300.0, 400.0),
            dpi: 100.0,
            title: None,
            facecolor: Color::rgb(255.0, 255.0, 255.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FigureBuilder;
    #[test]
    fn create_figure() {
        let fig = FigureBuilder::new().with_figsize(10, 20).build();
        assert_eq!(fig.f.figsize, (10.0 as f64, 20.0 as f64));
    }
}
