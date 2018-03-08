use backend::backend;

pub struct DummyBackend {
    fig: ()
}

impl backend::Backend for DummyBackend {
    type Err = !;
    type Figure = ();

    fn new() -> Self {
        DummyBackend { fig: () }
    }

    fn new_figure(&mut self, title: &str, size: &(f64, f64)) -> &mut Self::Figure {
        &mut self.fig
    }

    fn draw_path(&mut self, color: &(f64, f64, f64, f64), path: &[(f64, f64)]) -> Result<(), Self::Err> {
        Ok(())
    }

    fn show(self)-> Result<i32, Self::Err> {
        Ok(0)
    }
}
