use backend::Backend;
use matplotrs_backend::Backend as BackendTrait;
use matplotrs_backend;

pub trait Artist {
    /// Get collection of paths to draw this artist
    fn paths(&self) -> Vec<matplotrs_backend::Path>;

    /// Do nothing. Override if the type has children that should be rendered
    fn render_children(&self, _be: &mut Backend) -> Result<(), <Backend as BackendTrait>::Err> {
        Ok(())
    }
}
