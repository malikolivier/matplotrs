pub trait Backend {
    type Err;
    type Figure;
    fn new() -> Self;
    fn new_figure(&mut self, title: &str, size: &(f64, f64));
    fn draw_path(&mut self, color: &(f64, f64, f64, f64), path: &[(f64, f64)]) -> Result<(), Self::Err>;
    fn show(self) -> Result<i32, Self::Err>;
}
