use matplotrs_backend;

pub struct DummyBackend;

impl matplotrs_backend::Backend for DummyBackend {
    type Err = !;

    fn new() -> Self {
        DummyBackend
    }

    fn new_figure(
        &mut self,
        _: &matplotrs_backend::FigureRepr,
    ) -> Result<matplotrs_backend::FigureId, Self::Err> {
        Ok(matplotrs_backend::FigureId(0))
    }

    fn clear_figure(
        &mut self,
        _: matplotrs_backend::FigureId,
        _: &matplotrs_backend::FigureRepr,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_path(
        &mut self,
        _: matplotrs_backend::FigureId,
        _: &matplotrs_backend::Path,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_text(
        &mut self,
        _: matplotrs_backend::FigureId,
        _: &matplotrs_backend::Text,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_image(
        &mut self,
        _: matplotrs_backend::FigureId,
        _: &matplotrs_backend::Image,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn next_event(&mut self) -> Option<matplotrs_backend::Event> {
        None
    }

    fn save_to_file(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }
}
