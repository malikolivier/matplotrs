use matplotrs_backend;

pub struct DummyBackend {
    fig: (),
}

impl matplotrs_backend::Backend for DummyBackend {
    type Err = !;
    type Figure = ();

    fn new() -> Self {
        DummyBackend { fig: () }
    }

    fn new_figure(&mut self, title: &str, size: &(f64, f64)) {}

    fn draw_path(&mut self, path: &matplotrs_backend::Path) -> Result<(), Self::Err> {
        Ok(())
    }

    fn show(self) -> Result<i32, Self::Err> {
        Ok(0)
    }
}
