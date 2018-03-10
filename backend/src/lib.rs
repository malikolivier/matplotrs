pub trait Backend {
    type Err;
    type Figure;
    fn new() -> Self;
    fn new_figure(&mut self, title: &str, size: &(f64, f64));
    fn draw_path(&mut self, path: &Path) -> Result<(), Self::Err>;
    fn draw_text(&mut self, text: &Text) -> Result<(), Self::Err>;
    fn show(self) -> Result<i32, Self::Err>;
}

pub struct Path {
    pub points: Vec<(f64, f64)>,
    pub closed: bool,
    pub line_color: Option<(f64, f64, f64, f64)>,
    pub fill_color: Option<(f64, f64, f64, f64)>,
}

pub struct Text<'s> {
    pub point: (f64, f64),
    pub text: &'s str,
    pub font_size: f32,
}
