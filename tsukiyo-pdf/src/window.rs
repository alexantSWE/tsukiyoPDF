use image::{ImageBuffer, Rgb}; // Correct ImageBuffer import
use fltk::{app, enums::ColorDepth, frame::Frame, image::{self, RgbImage}, prelude::*, window::Window};
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
    // Convert `usize` to `u16` safely
    let page_number: u16 = page_number.try_into().ok()?; 

    // Initialize Pdfium
    let pdfium = Pdfium::new(Pdfium::bind_to_system_library().ok()?);
    
    // Load the PDF document
    let doc = pdfium.load_pdf_from_file(Path::new(pdf_path), None).ok()?;
    
    // Retrieve the specified page
    let page = doc.pages().get(page_number).ok()?; // Unwrap `Result`

    // Configure rendering
    let config = PdfRenderConfig::new()
        .set_target_width(800)
        .set_target_height(600);

    // Render the page
    let bitmap = page.render_with_config(&config).ok()?; //Now `page` is a `PdfPage`

    let (width, height) = (bitmap.width() as u32, bitmap.height() as u32);
    let bytes = bitmap.as_rgba_bytes(); // Get RGB byte slice

    // Convert raw bytes to an `RgbImage`
    let width: i32 = width as i32;
    let height: i32 = height as i32;

    RgbImage::new(
        bytes.as_slice(),          // Convert Vec<u8> to &[u8]
        height,                    // Image height
        width,    // Correct color type
        ColorDepth::Rgba8,                // Add the required ColorDepth argument
    ).ok()
    



 // Convert to Vec<u8> for ownership
}

fn main() {
    let pdf_path = "example.pdf";
    let page_number = 0;
    let width = 800;
    let height = 600;

    match render_pdf_page_to_image(pdf_path, page_number) {
        Some(raw_pixels) => {
            let img_buffer = ImageBuffer::<fltk::dialog::ColorMode, Vec<u8>>::from_raw(width, height, raw_pixels)
                .expect("Failed to create ImageBuffer");

            img_buffer.save("output.png").expect("Failed to save image");
        }
        None => eprintln!("Failed to render PDF page"),
    }
}

