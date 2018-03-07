pub struct Figure {
    f: FigureAttributes,
}

pub struct FigureBuilder {
    f: FigureAttributes,
}

struct FigureAttributes {
    figsize: (f64, f64),
    title: String,
}

impl FigureBuilder {
    fn new() -> Self {
        FigureBuilder { f: Default::default() }
    }

    fn with_figsize<W: Into<f64>, H: Into<f64>>(mut self, width: W, height: H) -> Self {
        self.f.figsize = (width.into(), height.into());
        self
    }

    fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.f.title = title.into();
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
            title: "Figure".to_owned(),
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
