pub trait Backend {
    type Err;
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

pub struct Text {
    pub point: (f64, f64),
    pub text: String,
    pub font_size: f32,
}
