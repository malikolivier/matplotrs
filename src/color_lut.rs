use color::Color;

pub enum ColorLUT {
    /// Linear gradient
    /// Takes a series of color stops that indicate how to interpolate between the colors
    LinearGradient(Vec<(f64, Color)>),
}

impl ColorLUT {
    /// Create a linear gradient.
    pub fn linear<T: Into<f64>>(colors: Vec<(T, Color)>) -> ColorLUT {
        let mut vec = Vec::with_capacity(colors.len());
        for (c, color) in colors {
            vec.push((c.into(), color))
        }
        ColorLUT::LinearGradient(vec)
    }

    pub fn color_at(&self, point: f64) -> Color {
        match self {
            &ColorLUT::LinearGradient(ref gradient) => {
                let first_color = gradient.iter();
                let next_color = gradient.iter().skip(1);
                for (&(v1, c1), &(v2, c2)) in first_color.zip(next_color) {
                    if point >= v1 {
                        let Color(r1, g1, b1, _) = c1;
                        let Color(r2, g2, b2, _) = c2;
                        let dv = v2 - v1;
                        let dp = point - v1;
                        let coef = dp / dv;
                        return Color::rgb(
                            r1 + (r2 - r1) * coef,
                            g1 + (g2 - g1) * coef,
                            b1 + (b2 - b1) * coef,
                        );
                    }
                }
                Color::rgb(0.0, 0.0, 0.0)
            }
        }
    }
}
