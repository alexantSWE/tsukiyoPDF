use fltk::{app, image::RgbImage, prelude::*, window::Window, frame::Frame};
use pdfium_render::prelude::*;
use log::info;
use std::path::Path;
//mod
pub fn create_window(pdf_path: &str) {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "PDF Viewer");

    let mut frame = Frame::new(0, 0, 800, 600, "");
    
    if let Some(image) = render_pdf_page_to_image(pdf_path, 0) { // Page 0
        frame.set_image(Some(image));
    } else {
        log::error!("Failed to render PDF page.");
    }

    wind.end();
    wind.show();
    app.run().unwrap();
}

fn render_pdf_page_to_image(pdf_path: &str, page_number: usize) -> Option<RgbImage> {
    let pdfium = Pdfium::new(Pdfium::bind_to_system_library().ok()?);
    let doc = pdfium.load_pdf_from_file(Path::new(pdf_path), None).ok()?;
    let page = doc.pages().get(page_number).ok()?;

    let bitmap = page.render_with_config(
        &PdfRenderConfig::new()
            .set_target_width(800)
            .set_target_height(600)
            .render()
    ).ok()?;

    let (width, height) = (bitmap.width() as i32, bitmap.height() as i32);
    let bytes = bitmap.as_rgb_bytes();

    RgbImage::new(&bytes, width, height, ColorDepth::Rgb8).ok()
}
