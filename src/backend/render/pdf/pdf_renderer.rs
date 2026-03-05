use printpdf::*;
use std::fs::{self, File};
use std::io::BufWriter;
use std::io::Write;

pub struct PdfRenderer;

impl PdfRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) -> Result<(), std::io::Error> {
        // Create document
        let mut doc = PdfDocument::new("Document");

        // Use built-in Helvetica font via PdfFontHandle
        let font_handle = PdfFontHandle::Builtin(BuiltinFont::Helvetica);

        // Create page with operations
        let ops = vec![
            Op::StartTextSection,
            Op::SetTextCursor {
                pos: Point::new(Mm(10.0), Mm(270.0)),
            },
            Op::SetFont {
                font: font_handle,
                size: Pt(48.0),
            },
            Op::ShowText {
                items: vec![TextItem::Text("Hello, PDF!".to_string())],
            },
            Op::EndTextSection,
        ];

        let page = PdfPage::new(Mm(210.0), Mm(297.0), ops);
        let pdf_bytes = doc
            .with_pages(vec![page])
            .save(&PdfSaveOptions::default(), &mut Vec::new());

        // Ensure generated directory exists
        fs::create_dir_all("generated")?;

        let file = File::create("generated/output.pdf")?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&pdf_bytes)?;

        Ok(())
    }
}
