use matplotrs_backend;
use artist::Artist;
use extend_vec::{self, HasMinMax};

pub struct Axis {
    axis_type: AxisType,
    pub lims: (f64, f64),
    visible: bool,
}

enum AxisType {
    XAxis,
    YAxis,
}
use self::AxisType::*;

impl Axis {
    pub fn new_xaxis(lims: (f64, f64)) -> Self {
        Self {
            axis_type: XAxis,
            lims: prevent_null_interval(lims),
            visible: true,
        }
    }

    pub fn new_yaxis(lims: (f64, f64)) -> Self {
        Self {
            axis_type: YAxis,
            lims: prevent_null_interval(lims),
            visible: true,
        }
    }

    pub fn new_xaxis_auto(data: &Vec<Vec<(f64, f64)>>) -> Self {
        let (&(x_min, _), &(x_max, _)) = data.min_max_with(extend_vec::tuple_partial_cmp_x)
            .unwrap_or((&(0.0, 0.0), &(0.0, 0.0)));
        Self::new_xaxis((x_min, x_max))
    }

    pub fn new_yaxis_auto(data: &Vec<Vec<(f64, f64)>>) -> Self {
        let (&(_, y_min), &(_, y_max)) = data.min_max_with(extend_vec::tuple_partial_cmp_y)
            .unwrap_or((&(0.0, 0.0), &(0.0, 0.0)));
        Self::new_yaxis((y_min, y_max))
    }

    /// Get relative coordinate of point in the contained axes (-1 to +1)
    pub fn world_coord_at<T>(&self, point: T) -> f64
    where
        T: Into<f64>,
    {
        let (min, max) = self.lims;
        match self.axis_type {
            XAxis => -1.0 + 2.0 * (point.into() - min) / (max - min),
            YAxis => 1.0 - 2.0 * (point.into() - min) / (max - min),
        }
    }

    /// Run a function over tick positions in the coordinates of the contained axes
    /// (-1 to +1) and the values of the ticks
    fn for_each_tick_positions<F>(&self, mut f: F)
    where
        F: FnMut(f64, f64),
    {
        if TICK_COUNT == 0 {
            return;
        }
        let (mut tick_pos, tick_step) = match self.axis_type {
            XAxis => (-1.0, TICK_STEP),
            YAxis => (1.0, -TICK_STEP),
        };
        let mut tick_val = self.lims.0;
        let tick_val_step = (self.lims.1 - self.lims.0) / TICK_COUNT as f64;
        for _ in 0..TICK_COUNT {
            f(tick_pos, tick_val);
            tick_pos += tick_step;
            tick_val += tick_val_step;
        }
    }
}

const TICK_COUNT: usize = 10;
const TICK_SIZE: f64 = 0.05;
const AXIS_COLOR: Option<(f64, f64, f64, f64)> = Some((0.0, 0.0, 0.0, 1.0));
const TICK_STEP: f64 = 2.0 / TICK_COUNT as f64;

const DEFAULT_FONT_SIZE: f32 = 10.0;

impl Artist for Axis {
    fn paths(&self) -> Vec<matplotrs_backend::Path> {
        if !self.visible {
            // Empty path
            Vec::new()
        } else {
            let mut paths = vec![
                // Axis line
                matplotrs_backend::Path {
                    points: match self.axis_type {
                        XAxis => vec![(-1.0, 1.0), (1.0, 1.0)],
                        YAxis => vec![(-1.0, 1.0), (-1.0, -1.0)],
                    },
                    closed: false,
                    line_color: AXIS_COLOR,
                    fill_color: None,
                },
            ];
            // Make path for each tick
            self.for_each_tick_positions(|tick_pos, _| {
                paths.push(matplotrs_backend::Path {
                    points: match self.axis_type {
                        XAxis => vec![(tick_pos, 1.0), (tick_pos, 1.0 + TICK_SIZE)],
                        YAxis => vec![(-1.0, tick_pos), (-1.0 - TICK_SIZE, tick_pos)],
                    },
                    closed: false,
                    line_color: AXIS_COLOR,
                    fill_color: None,
                });
            });
            paths
        }
    }

    fn texts(&self) -> Vec<matplotrs_backend::Text> {
        if !self.visible {
            Vec::new()
        } else {
            let mut texts = Vec::new();
            self.for_each_tick_positions(|tick_pos, tick_val| {
                texts.push(matplotrs_backend::Text {
                    point: match self.axis_type {
                        XAxis => (tick_pos, 1.0 + TICK_SIZE),
                        YAxis => (-1.0 - TICK_SIZE, tick_pos),
                    },
                    text: format!("{:.*}", 2, tick_val),
                    font_size: DEFAULT_FONT_SIZE,
                });
            });
            texts
        }
    }
}

fn prevent_null_interval((min, max): (f64, f64)) -> (f64, f64) {
    if max == min {
        (min - 0.5, max + 0.5)
    } else {
        (min, max)
    }
}
