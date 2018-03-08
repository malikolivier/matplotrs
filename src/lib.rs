pub mod axes;
pub mod figure;
pub mod color;
pub mod artist;

#[cfg(feature = "amethyst")]
pub extern crate matplotrs_amethyst_backend as amethyst_backend;

#[cfg(feature = "printpdf")]
pub extern crate matplotrs_printpdf_backend as printpdf_backend;
