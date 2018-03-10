use matplotrs_backend;
use artist::Artist;
use axis::Axis;

pub struct ImageView {
    data: Vec<Vec<f64>>,
    xaxis: Axis,
    yaxis: Axis,
    i: ImageViewAttributes,
}

pub struct ImageViewBuilder {
    data: Vec<Vec<f64>>,
    xlims: Option<(f64, f64)>,
    ylims: Option<(f64, f64)>,
    i: ImageViewAttributes,
}

pub struct ImageViewAttributes {}

impl ImageViewBuilder {
    pub fn new(image: Vec<Vec<f64>>) -> Self {
        Self {
            data: image,
            xlims: None,
            ylims: None,
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
                Some(ylims) => Axis::new_xaxis(ylims),
                None => Axis::new_xaxis((0.0, self.data[0].len() as f64)),
            };
            Ok(ImageView {
                data: self.data,
                xaxis,
                yaxis,
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
        Self {}
    }
}
