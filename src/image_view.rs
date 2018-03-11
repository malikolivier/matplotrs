use matplotrs_backend;
use artist::Artist;
use axis::Axis;
use color::{WHITE, BLACK, Color};
use color_lut::ColorLUT;

pub struct ImageView {
    data: Vec<Vec<f64>>,
    xaxis: Axis,
    yaxis: Axis,
    shape: (usize, usize),
    i: ImageViewAttributes,
}

pub struct ImageViewBuilder {
    data: Vec<Vec<f64>>,
    xlims: Option<(f64, f64)>,
    ylims: Option<(f64, f64)>,
    shape: (usize, usize),
    i: ImageViewAttributes,
}

pub struct ImageViewAttributes {
    interpolation: matplotrs_backend::Interpolation,
    lut: ColorLUT,
}

impl ImageViewBuilder {
    pub fn new(image: Vec<Vec<f64>>, shape: (usize, usize)) -> Self {
        Self {
            data: image,
            xlims: None,
            ylims: None,
            shape: shape,
            i: Default::default(),
        }
    }

    pub fn build(self) -> Result<ImageView, String> {
        if self.data.is_empty() || (!self.data.is_empty() && self.data[0].is_empty()) {
            Err("The provided array is empty!".to_owned())
        } else {
            let xaxis = match self.xlims {
                Some(xlims) => Axis::new_xaxis(xlims),
                None => Axis::new_xaxis((0.0, self.data.len() as f64)),
            };
            let yaxis = match self.ylims {
                Some(ylims) => Axis::new_yaxis(ylims),
                None => Axis::new_yaxis((0.0, self.data[0].len() as f64)),
            };
            Ok(ImageView {
                data: self.data,
                xaxis,
                yaxis,
                shape: self.shape,
                i: self.i,
            })
        }
    }

    pub fn with_xlims(mut self, xlims: (f64, f64)) -> Self {
        self.xlims = Some(xlims);
        self
    }

    pub fn with_ylims(mut self, ylims: (f64, f64)) -> Self {
        self.ylims = Some(ylims);
        self
    }
}

impl Default for ImageViewAttributes {
    fn default() -> Self {
        Self {
            interpolation: matplotrs_backend::Interpolation::None,
            lut: ColorLUT::linear(vec![(0.0, BLACK), (1.0, WHITE)]),
        }
    }
}

impl ImageView {
    fn raw_rgb(&self) -> Vec<u8> {
        let mut raw = Vec::with_capacity(3 * self.data.len());
        for line in self.data.iter() {
            for point in line.iter() {
                let bytes = self.i.lut.color_at(*point).bytes_rgb();
                raw.extend(&bytes);
            }
        }
        raw
    }
}

impl Artist for ImageView {
    fn paths(&self) -> Vec<matplotrs_backend::Path> {
        let mut paths = self.xaxis.paths();
        paths.extend(self.yaxis.paths());
        paths
    }

    fn texts(&self) -> Vec<matplotrs_backend::Text> {
        let mut texts = self.xaxis.texts();
        texts.extend(self.yaxis.texts());
        texts
    }

    fn images(&self) -> Vec<matplotrs_backend::Image> {
        vec![
            matplotrs_backend::Image {
                width: self.shape.0,
                height: self.shape.1,
                interpolation: self.i.interpolation,
                data: self.raw_rgb(),
                position: (-1.0, 1.0), // bottom-left corner
                size: (2.0, 2.0), // Fill complete axes
            },
        ]
    }
}
