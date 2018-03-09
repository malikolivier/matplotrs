use backend::Backend;
use matplotrs_backend::Backend as BackendTrait;
use matplotrs_backend;

pub trait Artist {
    fn paths(&self) -> Vec<matplotrs_backend::Path>;
    fn render_children(&self, _be: &mut Backend) -> Result<(), <Backend as BackendTrait>::Err> {
        Ok(())
    }
}
