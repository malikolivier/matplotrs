use matplotrs_backend;

pub trait Artist {
    fn path(&self) -> matplotrs_backend::Path;
}
