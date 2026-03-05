use crate::backend::render::pdf::pdf_renderer::PdfRenderer;

pub enum Renderer {
    Pdf,
    Epub,
    Wasm,
}

pub struct Backend {
    pub renderer: Renderer,
}

impl Backend {
    pub fn new(renderer: Renderer) -> Self {
        Self { renderer }
    }

    pub fn render(&self) -> Result<(), std::io::Error> {
        match self.renderer {
            Renderer::Pdf => {
                let renderer = PdfRenderer::new();
                renderer.render()
            }
            Renderer::Epub => todo!(),
            Renderer::Wasm => todo!(),
        }
    }
}
