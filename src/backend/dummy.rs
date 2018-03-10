use matplotrs_backend;

pub struct DummyBackend;

impl matplotrs_backend::Backend for DummyBackend {
    type Err = !;

    fn new() -> Self {
        DummyBackend
    }

    fn new_figure(&mut self, _title: &str, _size: &(f64, f64)) {}

    fn draw_path(&mut self, _path: &matplotrs_backend::Path) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_text(&mut self, _text: &matplotrs_backend::Text) -> Result<(), Self::Err> {
        Ok(())
    }

    fn show(self) -> Result<i32, Self::Err> {
        Ok(0)
    }
}
