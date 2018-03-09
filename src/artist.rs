use matplotrs_backend;

pub trait Artist {
    fn paths(&self) -> Vec<matplotrs_backend::Path>;
}
