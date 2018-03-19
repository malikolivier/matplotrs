use matplotrs_backend as mb;
use artist::Artist;
use axis::Axis;
use color::{BLACK, WHITE};
use color_lut::ColorLUT;
use extend_vec::{HasMinMax, HasShape, HasTotalLength};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use time;

pub struct ImageView {
    id: mb::ImageId,
    data: Vec<Vec<f64>>,
    xaxis: Axis,
    yaxis: Axis,
    vlims: (f64, f64),
    i: ImageViewAttributes,
}

pub struct ImageViewBuilder {
    data: Vec<Vec<f64>>,
    xlims: Option<(f64, f64)>,
    ylims: Option<(f64, f64)>,
    vlims: Option<(f64, f64)>,
    i: ImageViewAttributes,
}

pub struct ImageViewAttributes {
    interpolation: mb::Interpolation,
    lut: ColorLUT,
}

impl ImageViewBuilder {
    /// Create a new image view and deduce uts shape from input vector
    pub fn new(image: Vec<Vec<f64>>) -> Self {
        Self {
            data: image,
            xlims: None,
            ylims: None,
            vlims: None,
            i: Default::default(),
        }
    }

    /// Assumes data is row-major
    pub fn new_from_linear_data<T>(data: Vec<T>, shape: (usize, usize)) -> Result<Self, String>
    where
        T: Into<f64>,
    {
        let len = data.len();
        let (col_count, row_count) = shape;
        if len != col_count * row_count {
            Err("Data length and shape do not match!".to_owned())
        } else {
            let mut image = Vec::with_capacity(row_count);
            let mut row_vec = Vec::with_capacity(col_count);
            for (i, val) in data.into_iter().enumerate() {
                if i != 0 && (i % col_count) == 0 {
                    image.push(row_vec);
                    row_vec = Vec::with_capacity(col_count);
                }
                row_vec.push(val.into());
            }
            image.push(row_vec);
            Ok(Self::new(image))
        }
    }

    pub fn build(self) -> Result<ImageView, String> {
        if self.data.is_empty() || (!self.data.is_empty() && self.data[0].is_empty()) {
            Err("The provided array is empty!".to_owned())
        } else {
            let (width, height) = self.data.shape();
            let (disp_width, disp_height) = fit_size_within_container((width, height));
            let xaxis = match self.xlims {
                Some(xlims) => Axis::new_xaxis(xlims),
                None => Axis::new_xaxis((
                    0.0,
                    if width > height {
                        width as f64
                    } else {
                        width as f64 * 2.0 / disp_width
                    },
                )),
            };
            let yaxis = match self.ylims {
                Some(ylims) => Axis::new_yaxis(ylims),
                None => Axis::new_yaxis((
                    0.0,
                    if height > width {
                        height as f64
                    } else {
                        height as f64 * 2.0 / disp_height
                    },
                )),
            };
            let vlims = self.vlims.unwrap_or_else(|| {
                let (&vmin, &vmax) = self.data.min_max().unwrap_or((&0.0, &1.0));
                (vmin, vmax)
            });
            Ok(ImageView {
                id: compute_image_id(&self.data),
                data: self.data,
                xaxis,
                yaxis,
                vlims,
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

    pub fn with_vlims(mut self, vlims: (f64, f64)) -> Self {
        self.vlims = Some(vlims);
        self
    }
}

impl Default for ImageViewAttributes {
    fn default() -> Self {
        Self {
            interpolation: mb::Interpolation::None,
            lut: ColorLUT::linear(vec![(0.0, BLACK), (1.0, WHITE)]),
        }
    }
}

impl ImageView {
    pub fn raw_rgb(&self) -> Vec<u8> {
        let mut raw = Vec::with_capacity(3 * self.data.total_length());
        let (vmin, vmax) = self.vlims;
        for row in self.data.iter() {
            for val in row.iter() {
                let mut normalized_val = (*val - vmin) / (vmax - vmin);
                if normalized_val < 0.0 {
                    normalized_val = 0.0;
                } else if normalized_val > 1.0 {
                    normalized_val = 1.0;
                }
                let bytes = self.i.lut.color_at(normalized_val).bytes_rgb();
                raw.extend(&bytes);
            }
        }
        raw
    }

    /// Displayed size within container in container's coordinates
    fn world_size(&self) -> (f64, f64) {
        let (width, height) = self.data.shape();
        let (lower_left_pos_x, lower_left_pos_y) = self.world_position();
        let upper_right_pos_x = self.xaxis.world_coord_at(width as f64);
        let upper_right_pos_y = self.yaxis.world_coord_at(height as f64);
        // X axis in increasing in the up direction, while Y axis in decreasing in the right
        // direction. So to get the size,
        // we must take (- upper_right_pos_y + lower_left_pos_y), which is positive
        (
            upper_right_pos_x - lower_left_pos_x,
            -upper_right_pos_y + lower_left_pos_y,
        )
    }

    /// Displayed position (bottom-left corner) within container in container's coordinates
    fn world_position(&self) -> (f64, f64) {
        (self.xaxis.world_coord_at(0), self.yaxis.world_coord_at(0))
    }
}

/// Compute the displayed size so that a image of size (width x height) fits perfectly inside a
/// squared container (whose coordinates go from -1.0 to 1.0)
fn fit_size_within_container((width, height): (usize, usize)) -> (f64, f64) {
    if width > height {
        (2.0, 2.0 * height as f64 / width as f64)
    } else {
        (2.0 * width as f64 / height as f64, 2.0)
    }
}

impl Artist for ImageView {
    fn paths(&self) -> Vec<mb::Path> {
        let mut paths = self.xaxis.paths();
        paths.extend(self.yaxis.paths());
        paths
    }

    fn texts(&self) -> Vec<mb::Text> {
        let mut texts = self.xaxis.texts();
        texts.extend(self.yaxis.texts());
        texts
    }

    fn images(&self) -> Vec<mb::Image> {
        let (width, height) = self.data.shape();
        vec![
            mb::Image {
                id: self.id,
                width,
                height,
                interpolation: self.i.interpolation,
                data: self.raw_rgb(),
                position: self.world_position(),
                size: self.world_size(),
            },
        ]
    }
}

/// Make a unique ID to identify the image
fn compute_image_id(data: &Vec<Vec<f64>>) -> mb::ImageId {
    let mut s = DefaultHasher::new();
    for row in data.iter() {
        for val in row.iter() {
            (*val as u64).hash(&mut s);
        }
    }
    time::now().tm_nsec.hash(&mut s);
    mb::ImageId(s.finish())
}

#[cfg(test)]
mod tests {
    use super::ImageViewBuilder;
    use extend_vec::HasShape;

    #[test]
    fn deduce_correct_shape() {
        let image =
            ImageViewBuilder::new(vec![vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]; 100])
                .build()
                .unwrap();
        assert_eq!(image.data.shape(), (9, 100));
    }
}
