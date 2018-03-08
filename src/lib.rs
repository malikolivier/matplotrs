#![feature(never_type)]
pub mod axes;
pub mod figure;
pub mod color;
pub mod artist;

pub mod render;

#[cfg(feature = "amethyst")]
extern crate matplotrs_amethyst_backend as amethyst_backend;
#[cfg(feature = "amethyst")]
pub type Backend = amethyst_backend::AmethystBackend;

#[cfg(feature = "printpdf")]
extern crate matplotrs_printpdf_backend as printpdf_backend;
#[cfg(feature = "printpdf")]
pub type Backend = printpdf_backend::PrintpdfBackend;

#[cfg(not(all(feature = "amethyst", feature = "printpdf")))]
extern crate matplotrs_backend;
#[cfg(not(all(feature = "amethyst", feature = "printpdf")))]
pub mod dummy_backend;
#[cfg(not(all(feature = "amethyst", feature = "printpdf")))]
pub type Backend = dummy_backend::DummyBackend;
