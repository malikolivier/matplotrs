use color::Color;

pub struct Figure {
    f: FigureAttributes,
}

pub struct FigureBuilder {
    f: FigureAttributes,
}

struct FigureAttributes {
    figsize: (f64, f64),
    dpi: f64,
    title: Option<String>,
    facecolor: Color,
}

impl FigureBuilder {
    fn new() -> Self {
        FigureBuilder { f: Default::default() }
    }

    fn with_figsize<W: Into<f64>, H: Into<f64>>(mut self, width: W, height: H) -> Self {
        self.f.figsize = (width.into(), height.into());
        self
    }

    fn with_dpi<T: Into<f64>>(mut self, dpi: T) -> Self {
        self.f.dpi = dpi.into();
        self
    }

    fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.f.title = Some(title.into());
        self
    }

    fn with_facecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.f.facecolor = color.into();
        self
    }

    fn build(self) -> Figure {
        Figure { f: self.f }
    }
}

impl Figure {
    fn new() -> Self {
        FigureBuilder::new().build()
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
