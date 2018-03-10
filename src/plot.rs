use matplotrs_backend;
use color::{Color, BLACK};
use artist::Artist;
use axis::Axis;

pub struct Plot {
    data: Vec<PlotSeries>,
    xaxis: Axis,
    yaxis: Axis,
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

impl PlotBuilder {
    /// Make a new plot builder with a single series
    pub fn new(one_series: Vec<(f64, f64)>) -> Self {
        Self {
            data: vec![one_series],
            xlims: None,
            ylims: None,
            p: Default::default(),
        }
    }

    pub fn build(self) -> Plot {
        let xaxis = match self.xlims {
            Some(xlims) => Axis::new_xaxis(xlims),
            None => Axis::new_xaxis_auto(&self.data),
        };
        let yaxis = match self.ylims {
            Some(ylims) => Axis::new_xaxis(ylims),
            None => Axis::new_yaxis_auto(&self.data),
        };
        let all_series = self.data.into_iter().map(|one_series|
            PlotSeries { data: one_series, edgecolor: BLACK }
        ).collect();
        Plot {
            data: all_series,
            xaxis,
            yaxis,
            p: self.p,
        }
    }

    /// Make a new plot builder starting with several series
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

    /// Add a new series to the plot
    pub fn with_new_series(mut self, one_series: Vec<(f64, f64)>) -> Self {
        self.data.push(one_series);
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
        let &(xmin, xmax) = self.xlims();
        let &(ymin, ymax) = self.ylims();
        for point in path.points.iter_mut() {
            let (px, py) = *point;
            *point = (
                (px - xmin) * 2.0 / (xmax - xmin) - 1.0,
                -(py - ymin) * 2.0 / (ymax - ymin) + 1.0,
            );
        }
        path
    }

    fn xlims(&self) -> &(f64, f64) {
        &self.xaxis.lims
    }

    fn ylims(&self) -> &(f64, f64) {
        &self.yaxis.lims
    }
}
