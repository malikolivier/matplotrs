#![feature(never_type)]
#![feature(slice_patterns)]
pub mod axes;
pub mod figure;
pub mod color;
pub mod artist;

pub mod app;
pub mod line;
pub mod line_collection;
pub mod plot;
mod axis;
pub mod image_view;
pub mod color_lut;
mod extend_vec;

pub extern crate matplotrs_backend;

#[cfg(feature = "amethyst")]
pub mod backend {
    pub extern crate matplotrs_amethyst_backend as backend;
    pub type Backend = backend::AmethystBackend;
}

#[cfg(feature = "printpdf")]
pub mod backend {
    pub extern crate matplotrs_printpdf_backend as backend;
    pub type Backend = backend::PrintPdfBackend;
}

#[cfg(not(any(feature = "amethyst", feature = "printpdf")))]
pub mod backend {
    mod dummy;
    pub type Backend = dummy::DummyBackend;
}
