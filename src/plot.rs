use matplotrs_backend;
use color::{Color, BLACK};
use std::cmp::Ordering;
use artist::Artist;

pub struct Plot {
    data: Vec<PlotSeries>,
    xlims: (f64, f64),
    ylims: (f64, f64),
    p: PlotAttributes,
}

pub struct PlotSeries {
    data: Vec<(f64, f64)>,
    edgecolor: Color,
}

pub struct PlotBuilder {
    data: Vec<Vec<(f64, f64)>>,
    xlims: Option<(f64, f64)>,
    ylims: Option<(f64, f64)>,
    p: PlotAttributes,
}

pub struct PlotAttributes {
}

trait MinMaxWith<T>: IntoIterator<Item = T> {
    fn min_with<F>(&self, f: F) -> Option<&T>
    where
        F: Fn(&T, &T) -> Ordering;
    fn max_with<F>(&self, f: F) -> Option<&T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        self.min_with(|x1, x2| f(x1, x2).reverse())
    }
}

impl<T> MinMaxWith<T> for Vec<T> {
    fn min_with<F>(&self, f: F) -> Option<&T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        if self.is_empty() {
            None
        } else {
            let vec = self.as_slice();
            let mut min = &vec[0];
            for item in vec.iter().skip(1) {
                if let Ordering::Less = f(item, &min) {
                    min = item;
                }
            }
            Some(min)
        }
    }
}

fn tuple_partial_cmp_x(&(x1, _y1): &(f64, f64), &(x2, _y2): &(f64, f64)) -> Ordering {
    x1.partial_cmp(&x2).unwrap_or(Ordering::Less)
}

fn tuple_partial_cmp_y(&(_x1, y1): &(f64, f64), &(_x2, y2): &(f64, f64)) -> Ordering {
    y1.partial_cmp(&y2).unwrap_or(Ordering::Less)
}

fn x_min_max(series: &Vec<Vec<(f64, f64)>>) -> (f64, f64) {
    let mut min = 0.0;
    let mut max = 0.0;
    for single_series in series {
        single_series.min_with(tuple_partial_cmp_x).map(|&(x_min,
           _y_min)| {
            min = x_min;
        });
        single_series.max_with(tuple_partial_cmp_x).map(|&(x_max,
           _y_max)| {
            max = x_max;
        });
    }
    (min, max)
}

fn y_min_max(series: &Vec<Vec<(f64, f64)>>) -> (f64, f64) {
    let mut min = 0.0;
    let mut max = 0.0;
    for single_series in series {
        single_series.min_with(tuple_partial_cmp_y).map(|&(_x_min,
           y_min)| {
            min = y_min;
        });
        single_series.max_with(tuple_partial_cmp_y).map(|&(_x_max,
           y_max)| {
            max = y_max;
        });
    }
    (min, max)
}

fn prevent_null_interval((min, max): (f64, f64)) -> (f64, f64) {
    if max == min {
        (min - 0.5, max + 0.5)
    } else {
        (min, max)
    }
}

impl PlotBuilder {
    pub fn new(one_series: Vec<(f64, f64)>) -> Self {
        Self {
            data: vec![one_series],
            xlims: None,
            ylims: None,
            p: Default::default(),
        }
    }

    pub fn build(self) -> Plot {
        let xlims = match self.xlims {
            Some(xlims) => xlims,
            None => x_min_max(&self.data),
        };
        let ylims = match self.ylims {
            Some(ylims) => ylims,
            None => y_min_max(&self.data),
        };
        let all_series = self.data.into_iter().map(|one_series|
            PlotSeries { data: one_series, edgecolor: BLACK }
        ).collect();
        Plot {
            data: all_series,
            xlims: prevent_null_interval(xlims),
            ylims: prevent_null_interval(ylims),
            p: self.p,
        }
    }

    pub fn new_multi_series(multi_series: Vec<Vec<(f64, f64)>>) -> Self {
        Self {
            data: multi_series,
            xlims: None,
            ylims: None,
            p: Default::default(),
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

impl Default for PlotAttributes {
    fn default() -> Self {
        Self {}
    }
}

impl Artist for Plot {
    fn paths(&self) -> Vec<matplotrs_backend::Path> {
        self.data
            .iter()
            .map(|series| {
                let Color(r, g, b, a) = series.edgecolor;
                let path = matplotrs_backend::Path {
                    points: series.data.clone(),
                    closed: false,
                    line_color: Some((r, g, b, a)),
                    fill_color: None,
                };
                self.transform_path(path)
            })
            .collect()
    }
}

impl Plot {
    /// Transform plot path to make xlims and ylims fit into [-1, 1]
    fn transform_path(&self, mut path: matplotrs_backend::Path) -> matplotrs_backend::Path {
        let (xmin, xmax) = self.xlims;
        let (ymin, ymax) = self.ylims;
        for point in path.points.iter_mut() {
            let (px, py) = *point;
            *point = (
                (px - xmin) * 2.0 / (xmax - xmin) - 1.0,
                -(py - ymin) * 2.0 / (ymax - ymin) + 1.0,
            );
        }
        path
    }
}
