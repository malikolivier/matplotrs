pub extern crate matplotrs_backend;
pub use matplotrs_backend::Backend;
extern crate printpdf;

use std::fs::File;
use std::io::BufWriter;

use printpdf::{PdfDocument, PdfDocumentReference, PdfLayerReference, Mm, BuiltinFont, IndirectFontRef};

pub struct PrintPdfBackend {
    doc: Option<PdfDocumentReference>,
    layer: Option<PdfLayerReference>,
    size: Option<(Mm, Mm)>,
    default_font: Option<IndirectFontRef>,
}

#[derive(Debug)]
pub enum PdfError {
    BackEndError(String),
    PrintPdfError(printpdf::PrintpdfError),
    IOError(std::io::Error),
}

const DEFAULT_FONT: BuiltinFont = BuiltinFont::TimesRoman;

impl matplotrs_backend::Backend for PrintPdfBackend {
    type Err = PdfError;
    fn new() -> Self {
        PrintPdfBackend {
            doc: None,
            layer: None,
            size: None,
            default_font: None,
        }
    }

    fn new_figure(&mut self, title: &str, size: &(f64, f64)) -> Result<(), Self::Err> {
        match self.doc {
            None => {
                let (doc, page1, layer1) = PdfDocument::new(title, Mm(size.0), Mm(size.1), "Layer 1");
                let layer = doc.get_page(page1).get_layer(layer1);
                let default_font = doc.add_builtin_font(DEFAULT_FONT)?;
                self.doc = Some(doc);
                self.layer = Some(layer);
                self.default_font = Some(default_font);
            },
            Some(ref mut doc) => {
                let (new_page, new_layer1) = doc.add_page(Mm(size.0), Mm(size.1), title);
                self.layer = Some(doc.get_page(new_page).get_layer(new_layer1));
            }
        }
        self.size = Some((Mm(size.0), Mm(size.1)));
        Ok(())
    }

    fn draw_path(&mut self, path: &matplotrs_backend::Path) -> Result<(), Self::Err> {
        let points = path.points.iter().map(|coords| {
            let (x_pdf, y_pdf) = self.transform(coords);
            (printpdf::Point::new(x_pdf, y_pdf), false)
        }).collect();
        let line = printpdf::Line {
            points,
            is_closed: path.closed,
            has_fill: path.fill_color.is_some(),
            has_stroke: path.line_color.is_some(),
            is_clipping_path: false,
        };
        let layer = self.layer.as_ref().ok_or_else(|| {
            PdfError::BackEndError("No figure created!".to_owned())
        })?;
        if let Some(color) = path.fill_color {
            let fill_color = printpdf::Color::Rgb(printpdf::Rgb::new(color.0, color.1, color.2, None));
            layer.set_fill_color(fill_color);
        }
        if let Some(color) = path.line_color {
            let line_color = printpdf::Color::Rgb(printpdf::Rgb::new(color.0, color.1, color.2, None));
            layer.set_outline_color(line_color);
        }
        layer.add_shape(line);
        Ok(())
    }

    fn draw_text(&mut self, text: &matplotrs_backend::Text) -> Result<(), Self::Err> {
        match (self.layer.as_ref(), self.default_font.as_ref()) {
            (None, _) => Err(PdfError::BackEndError("No layer!".to_owned())),
            (_, None) => Err(PdfError::BackEndError("No font!".to_owned())),
            (Some(layer), Some(font)) => {
                let (x_pdf, y_pdf) = self.transform(&text.point);
                layer.begin_text_section();
                layer.set_font(&font, text.font_size as i64);
                layer.set_text_cursor(x_pdf, y_pdf);
                layer.write_text(text.text.as_str(), &font);
                layer.end_text_section();
                Ok(())
            }
        }
    }

    fn draw_image(&mut self, image: &matplotrs_backend::Image) -> Result<(), Self::Err> {
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
