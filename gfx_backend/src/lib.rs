extern crate matplotrs_backend as mb;

pub struct GfxBackend;

impl mb::Backend for GfxBackend {
    type Err = GfxError;

    fn new() -> Self {
        GfxBackend
    }

    fn new_figure(
        &mut self,
        _: &mb::FigureRepr,
    ) -> Result<mb::FigureId, Self::Err> {
        Ok(mb::FigureId(0))
    }

    fn clear_figure(
        &mut self,
        _: mb::FigureId,
        _: &mb::FigureRepr,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_path(
        &mut self,
        _: mb::FigureId,
        _: &mb::Path,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_text(
        &mut self,
        _: mb::FigureId,
        _: &mb::Text,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn draw_image(
        &mut self,
        _: mb::FigureId,
        _: &mb::Image,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn next_event(&mut self) -> Option<mb::Event> {
        None
    }

    fn save_to_file(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct GfxError;

impl<'a> From<&'a str> for GfxError {
    fn from(_: &str) -> Self {
        GfxError
    }
}

impl From<String> for GfxError {
    fn from(_: String) -> Self {
        GfxError
    }
}
