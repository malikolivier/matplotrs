extern crate matplotrs_backend;
extern crate printpdf;

use std::fs::File;
use std::io::BufWriter;

use printpdf::{PdfDocument, PdfDocumentReference, Mm};
// use printpdf::indices::{PdfPageIndex, PdfLayerIndex};

pub struct PrintPdfBackend {
    doc: Option<PdfDocumentReference>
}

pub enum PdfError {
    BackEndError(String),
    PrintPdfError(printpdf::PrintpdfError),
    IOError(std::io::Error),
}

impl matplotrs_backend::Backend for PrintPdfBackend {
    type Err = PdfError;
    type Figure = PdfDocumentReference;
    fn new() -> Self {
        PrintPdfBackend {
            doc: None
        }
    }

    fn new_figure(&mut self, title: &str, size: &(f64, f64)) -> &mut Self::Figure {
        match self.doc {
            None => {
                let (doc, _page1, _layer1) = PdfDocument::new(title, Mm(size.0), Mm(size.1), "Layer 1");
                self.doc = Some(doc);
                self.doc.as_mut().unwrap()
            },
            Some(ref mut doc) => {
                let (_page2, _layer1) = doc.add_page(Mm(size.0), Mm(size.1), title);
                doc
            }
        }

    }

    fn draw_path(&mut self, color: &(f64, f64, f64, f64), path: &[(f64, f64)]) -> Result<(), Self::Err> {
        unimplemented!()
    }

    fn show(self)-> Result<i32, Self::Err> {
        match self.doc {
            None => Err(PdfError::BackEndError("No figure created!".to_owned())),
            Some(doc) => {
                let mut writer = BufWriter::new(File::create("out.pdf")?);
                doc.save(&mut writer)?;
                Ok(0)
            }
        }

    }
}


impl From<std::io::Error> for PdfError {
    fn from(err: std::io::Error) -> Self {
        PdfError::IOError(err)
    }
}

impl From<printpdf::PrintpdfError> for PdfError {
    fn from(err: printpdf::PrintpdfError) -> Self {
        PdfError::PrintPdfError(err)
    }
}
