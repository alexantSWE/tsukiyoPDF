// For the external image crate, use a namespace alias
use image as img;
use fltk::{app, enums::ColorDepth, frame::Frame, image::{self, RgbImage}, prelude::*, window::Window};
use pdfium_render::prelude::*;
use log::info;
use std::path::Path;

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
    let page = doc.pages().get(page_number).ok()?;

    // Configure rendering
    let config = PdfRenderConfig::new()
        .set_target_width(800)
        .set_target_height(600);

    // Render the page
    let bitmap = page.render_with_config(&config).ok()?;

    let (width, height) = (bitmap.width() as u32, bitmap.height() as u32);
    let bytes = bitmap.as_rgba_bytes(); // Get RGBA byte slice

    // Convert raw bytes to an `RgbImage`
    let width: i32 = width as i32;
    let height: i32 = height as i32;

    RgbImage::new(
        bytes.as_slice(),
        width,                     // Width should be first
        height,                    // Height should be second
        image::ColorType::Rgba8,   // Correct color type
        ColorDepth::Rgba8,         // Add the required ColorDepth argument
    ).ok()
}

fn main() {
    let pdf_path = "example.pdf";
    let page_number = 0;

    // For display in window
    create_window(pdf_path);

    // For saving to file
    if let Some(fltk_image) = render_pdf_page_to_image(pdf_path, page_number) {
        // Convert FLTK image to image crate's format for saving
        // Note: This part needs to be adjusted based on how the data is formatted
        let width = fltk_image.width() as u32;
        let height = fltk_image.height() as u32;
        let data = fltk_image.to_rgb_data(); // Get RGB data from FLTK image
        
        let img_buffer = img::RgbImage::from_raw(width, height, data)
            .expect("Failed to create ImageBuffer");

        img_buffer.save("output.png").expect("Failed to save image");
    } else {
        eprintln!("Failed to render PDF page");
    }
}
