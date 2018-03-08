#![feature(never_type)]
pub mod axes;
pub mod figure;
pub mod color;
pub mod artist;

pub mod app;
pub mod render;

#[cfg(feature = "amethyst")]
pub mod backend {
    extern crate matplotrs_amethyst_backend as backend;
    type Backend = backend::AmethystBackend;
}

#[cfg(feature = "printpdf")]
pub mod backend {
    extern crate matplotrs_printpdf_backend as backend;
    type Backend = backend::PrintpdfBackend;
}

#[cfg(not(all(feature = "amethyst", feature = "printpdf")))]
pub mod backend {
    extern crate matplotrs_backend as backend;
    pub mod dummy;
    type Backend = dummy::DummyBackend;
}
