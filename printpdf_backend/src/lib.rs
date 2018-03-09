pub extern crate matplotrs_backend;
pub use matplotrs_backend::Backend;
extern crate printpdf;

use std::fs::File;
use std::io::BufWriter;

use printpdf::{PdfDocument, PdfDocumentReference, PdfLayerReference, Mm};

pub struct PrintPdfBackend {
    doc: Option<PdfDocumentReference>,
    layer: Option<PdfLayerReference>,
    size: Option<(Mm, Mm)>,
}

#[derive(Debug)]
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
            doc: None,
            layer: None,
            size: None,
        }
    }

    fn new_figure(&mut self, title: &str, size: &(f64, f64)) {
        match self.doc {
            None => {
                let (doc, page1, layer1) = PdfDocument::new(title, Mm(size.0), Mm(size.1), "Layer 1");
                let layer = doc.get_page(page1).get_layer(layer1);
                self.doc = Some(doc);
                self.layer = Some(layer);
            },
            Some(ref mut doc) => {
                let (new_page, new_layer1) = doc.add_page(Mm(size.0), Mm(size.1), title);
                self.layer = Some(doc.get_page(new_page).get_layer(new_layer1));
            }
        }
        self.size = Some((Mm(size.0), Mm(size.1)));
    }

    fn draw_path(&mut self, color: &(f64, f64, f64, f64), path: &[(f64, f64)]) -> Result<(), Self::Err> {
        let points = path.iter().map(|coords| {
            let (x_pdf, y_pdf) = self.transform(coords);
            (printpdf::Point::new(x_pdf, y_pdf), false)
        }).collect();
        let line = printpdf::Line {
            points,
            is_closed: true,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        };
        // let fill_color = printpdf::Color::Rgb(printpdf::Rgb::new(color.0, color.1, color.2, None));
        // let outline_color = printpdf::Color::Rgb(printpdf::Rgb::new(0.0, 0.0, 0.0, None));
        self.layer.as_ref().ok_or_else(|| {
            PdfError::BackEndError("No figure created!".to_owned())
        })?.add_shape(line);
        Ok(())
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

impl PrintPdfBackend {
    fn transform(&self, &(x, y): &(f64, f64)) -> (Mm, Mm) {
        let (Mm(rightmost), Mm(upmost)) = self.size.expect("Some size");
        (Mm(rightmost * (1.0 + x) / 2.0), Mm(upmost * (1.0 - y) / 2.0))
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
