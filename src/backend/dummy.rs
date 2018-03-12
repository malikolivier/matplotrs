use matplotrs_backend;

pub struct DummyBackend;

impl matplotrs_backend::Backend for DummyBackend {
    type Err = !;

    fn new() -> Self {
        DummyBackend
    }

    fn new_figure(&mut self, _title: &str, _size: &(f64, f64)) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_path(&mut self, _: &matplotrs_backend::Path) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_text(&mut self, _: &matplotrs_backend::Text) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_image(&mut self, _: &matplotrs_backend::Image) -> Result<(), Self::Err> {
        Ok(())
    }

    fn next_event(&mut self) -> Option<matplotrs_backend::Event> {
        None
    }

    fn save_to_file(&mut self)-> Result<(), Self::Err> {
        Ok(())
    }

    fn show(self) -> Result<i32, Self::Err> {
        Ok(0)
    }
}
