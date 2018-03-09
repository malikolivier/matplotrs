// use line_collection::LineCollection;
use color::{Color, BLACK};
use std::cmp::Ordering;

pub struct Plot {
    data: Vec<Vec<(f64, f64)>>,
    xlims: (f64, f64),
    ylims: (f64, f64),
    p: PlotAttributes,
}

pub struct PlotBuilder {
    data: Vec<Vec<(f64, f64)>>,
    xlims: Option<(f64, f64)>,
    ylims: Option<(f64, f64)>,
    p: PlotAttributes,
}

pub struct PlotAttributes {
    edgecolor: Color,
}

trait MinMaxWith: IntoIterator<Item = (f64, f64)> {
    fn min_with<F>(&self, f: F) -> Option<(f64, f64)>
        where F: Fn(&(f64, f64), &(f64, f64)) -> Ordering;
    fn max_with<F>(&self, f: F) -> Option<(f64, f64)>
        where F: Fn(&(f64, f64), &(f64, f64)) -> Ordering
    {
        self.min_with(|x1, x2| f(x1, x2).reverse())
    }
}

impl MinMaxWith for Vec<(f64,f64)> {
    fn min_with<F>(&self, f: F) -> Option<(f64, f64)>
        where F: Fn(&(f64, f64), &(f64, f64)) -> Ordering
    {
        if self.is_empty() {
            None
        } else {
            let vec = self.as_slice();
            let mut min = vec[0];
            for item in vec.iter().skip(1) {
                if let Ordering::Less = f(item, &min) {
                    min = *item;
                }
            }
            Some(min)
        }
    }
}

fn tuple_partial_cmp_x(&(x1, _y1): &(f64, f64), &(x2, _y2): &(f64, f64)) -> Ordering {
    x1.partial_cmp(&x2).unwrap_or(Ordering::Less)
}

fn x_min_max(series: &Vec<Vec<(f64, f64)>>) -> (f64, f64) {
    let mut min = 0.0;
    let mut max = 0.0;
    for single_series in series {
        single_series.min_with(tuple_partial_cmp_x).map(|(x_min, _y_min)| {
            min = x_min;
        });
        single_series.max_with(tuple_partial_cmp_x).map(|(x_max, _y_max)| {
            max = x_max;
        });
    }
    (min, max)
}

impl PlotBuilder {
    pub fn new(one_series: Vec<(f64, f64)>) -> Self {
        Self { data: vec![one_series], xlims: None, ylims: None, p: Default::default() }
    }

    pub fn build(self) -> Plot {
        let xlims = match self.xlims {
            Some(xlims) => xlims,
            None        => x_min_max(&self.data),
        };
        let ylims = match self.xlims {
            Some(xlims) => xlims,
            None        => (0.0, 0.0),
        };
        Plot {
            data: self.data,
            xlims,
            ylims,
            p: self.p,
        }
    }

    pub fn new_multi_series(multi_series: Vec<Vec<(f64, f64)>>) -> Self {
        Self { data: multi_series, xlims: None, ylims: None, p: Default::default() }
    }

    pub fn with_xlims(mut self, xlims: (f64, f64)) -> Self {
        self.xlims = Some(xlims);
        self
    }

    pub fn with_ylims(mut self, ylims: (f64, f64)) -> Self {
        self.ylims = Some(ylims);
        self
    }

    pub fn with_edgecolor<T: Into<Color>>(mut self, color: T) -> Self {
        self.p.edgecolor = color.into();
        self
    }
}

impl Default for PlotAttributes {
    fn default() -> Self {
        Self {
            edgecolor: BLACK,
        }
    }
}
