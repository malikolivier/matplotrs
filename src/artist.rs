pub trait Artist {
    fn path(&self) -> (Vec<(f64, f64)>, bool);
}
