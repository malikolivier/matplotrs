use color::Color;
use axes::AxesBuilder;
use artist::Artist;
use app::App;

pub struct Figure {
    f: FigureAttributes,
    children: Vec<Box<Artist>>,
}

pub struct FigureBuilder<'a> {
    pub f: FigureAttributes,
    pub app: &'a mut App,
}

pub struct FigureAttributes {
    figsize: (f64, f64),
    dpi: f64,
    title: Option<String>,
    facecolor: Color,
}

impl<'a> FigureBuilder<'a> {
    pub fn with_figsize<W: Into<f64>, H: Into<f64>>(mut self, width: W, height: H) -> Self {
        self.f.figsize = (width.into(), height.into());
        self
    }

    pub fn with_dpi<T: Into<f64>>(mut self, dpi: T) -> Self {
        self.f.dpi = dpi.into();
        self
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.f.title = Some(title.into());
        self
    }

    pub fn with_facecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.f.facecolor = color.into();
        self
    }

    pub fn build(self) -> &'a mut Figure {
        let fig = Figure { f: self.f, children: Vec::new() };
        self.app.figs.push(fig);
        self.app.figs.last_mut().unwrap()
    }
}

impl Figure {
    pub fn add_axes(&mut self) -> AxesBuilder {
        AxesBuilder { fig: self, a: Default::default() }
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
