#![feature(never_type)]
pub mod axes;
pub mod figure;
pub mod color;
pub mod artist;

pub mod app;

#[cfg(feature = "amethyst")]
pub mod backend {
    pub extern crate matplotrs_amethyst_backend as backend;
    type Backend = backend::AmethystBackend;
}

#[cfg(feature = "printpdf")]
pub mod backend {
    pub extern crate matplotrs_printpdf_backend as backend;
    type Backend = backend::PrintpdfBackend;
}

#[cfg(not(all(feature = "amethyst", feature = "printpdf")))]
pub mod backend {
    pub extern crate matplotrs_backend as backend;
    mod dummy;
    pub type Backend = dummy::DummyBackend;
}
