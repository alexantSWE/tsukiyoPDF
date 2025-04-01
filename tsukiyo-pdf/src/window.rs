use image::{ImageBuffer, Rgb};
use fltk::{app, enums::ColorDepth, frame::Frame, image::RgbImage as FltkRgbImage, prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt}, window::Window};
use pdfium_render::prelude::*;
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




fn render_pdf_page_to_image(pdf_path: &str, page_number: usize) -> Option<FltkRgbImage> {
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
    let bitmap = page.render_with_config(&config).ok()?; // Now `bitmap` is a `PdfBitmap`

    let (width, height) = (bitmap.width() as i32, bitmap.height() as i32);
    let bytes = bitmap.as_rgba_bytes(); // Get RGBA byte slice

    // Convert raw bytes to an `FltkRgbImage`
    let rgb_image = FltkRgbImage::new(
        bytes.as_slice(),  // Convert Vec<u8> to &[u8]
        width,             // Image width
        height,            // Image height
        ColorDepth::Rgba8, // Specify RGBA format
    ).ok(); // <- THIS ENSURES `Option<FltkRgbImage>` IS RETURNED 

    }

 /*
Convert to Vec<u8> for ownership
*/


fn main() {
    let pdf_path = "example.pdf";
    let page_number = 0;
    let width = 800;
    let height = 600;

    match render_pdf_page_to_image(pdf_path, page_number) {
        Some(fltk_img) => {
            let raw_pixels = fltk_img.to_rgb_data();
            let img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, raw_pixels)
                .expect("Failed to create image buffer");
    
            img.save("output.png").expect("Failed to save image");
        }
        None => eprintln!("Failed to render PDF page"),
    } // Corrected the trailing comma
}
