pub mod axes;
pub mod figure;
pub mod color;
pub mod artist;

#[cfg(feature = "amethyst")]
pub extern crate matplotrs_amethyst_backend as amethyst_backend;
