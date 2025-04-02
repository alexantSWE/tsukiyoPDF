use fltk::{
    app,
    enums::ColorDepth,
    frame::Frame,
    image::RgbImage as FltkRgbImage,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use pdfium_render::prelude::*;
use std::path::Path;
// NOTE: Removed 'image' crate import as it's not used in the GUI part.
//       Add it back if you uncomment the saving logic in main.

// Function to create and run the FLTK window
pub fn create_window(pdf_path: &str) {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "PDF Viewer");

    // Create a frame that will hold the rendered PDF page image
    let mut frame = Frame::new(0, 0, 800, 600, "");

    // Attempt to render the first page (index 0) of the PDF
    match render_pdf_page_to_image(pdf_path, 0) {
        Some(image) => {
            // If rendering is successful, set the image to the frame
            // Note: FltkRgbImage's internal data might need scaling if its
            // dimensions don't match the frame. Frame usually scales automatically.
            frame.set_image(Some(image));
            // Redraw the frame to display the image
            frame.redraw();
        }
        None => {
            // If rendering fails, print an error message to stderr
            eprintln!("Error: Failed to render PDF page.");
            // Optionally display an error message in the frame itself
            frame.set_label("Failed to load PDF page.");
        }
    }

    wind.end(); // End adding widgets to the window
    wind.show(); // Make the window visible
    app.run().unwrap(); // Start the FLTK event loop
}

// Function to render a specific page of a PDF to an FLTK image
fn render_pdf_page_to_image(pdf_path: &str, page_number: usize) -> Option<FltkRgbImage> {
    // Convert page number from usize to u16, return None on failure (e.g., overflow)
    let page_number_u16: u16 = page_number.try_into().ok()?;

    // Initialize the Pdfium library bindings. Return None if binding fails.
    // This assumes the Pdfium library is available on the system.
    let pdfium = Pdfium::new(Pdfium::bind_to_system_library().ok()?);

    // Load the PDF document from the given path. Return None on failure.
    let doc = pdfium.load_pdf_from_file(Path::new(pdf_path), None).ok()?;

    // Get the specific page from the document. Return None if the page number is invalid.
    let page = doc.pages().get(page_number_u16).ok()?; // Get page as Result, convert to Option, propagate None

    // Configure the rendering options. Here, we aim for a specific size.
    // Aspect ratio might not be preserved if target dimensions differ from page ratio.
    let render_config = PdfRenderConfig::new()
        .set_target_width(800) // Target width for rendering
        .set_target_height(600); // Target height for rendering
        // Add .force_halftone(true) if rendering quality is poor on some viewers
        // Add .render_for_printing(true) for potentially higher quality rendering
        // Consider set_clear_white(true) or background color if needed

    // Render the page to a bitmap with the specified configuration. Return None on failure.
    let bitmap = page.render_with_config(&render_config).ok()?;

    // Get the dimensions (width and height) of the rendered bitmap.
    let width = bitmap.width() as i32; // Cast to i32 for FLTK
    let height = bitmap.height() as i32; // Cast to i32 for FLTK

    // Get the pixel data as a slice of bytes (Option<&[u8]>).
    // Assumes RGBA format based on ColorDepth::Rgba8 used below.
    // Pdfium might produce BGRA, check color output!
    let bytes = bitmap.as_rgba_bytes(); // Propagate None if conversion fails

    // Create an FltkRgbImage from the raw pixel data.
    // FltkRgbImage::new copies the data, so the 'bytes' slice doesn't need to outlive this function.
    // We specify Rgba8 (4 bytes per pixel). Check if colors are swapped (BGRA issue).
          
// Create an FltkRgbImage from the raw pixel data.
    // FltkRgbImage::new copies the data from the slice.
    let fltk_image = FltkRgbImage::new(
        &bytes,            // Pass a slice &[u8] of the Vec<u8>
        width,             // Image width
        height,            // Image height
        ColorDepth::Rgba8, // Specify RGBA format
    )
    .ok()?; // Convert Result<FltkRgbImage, FltkError> to Option<FltkRgbImage>

    

    Some(fltk_image) // Return the created FLTK image wrapped in Some
} // Added missing closing brace

// Main function - entry point of the application
fn main() {
    // Define the path to the PDF file
    // Make sure this file exists relative to where you run the executable!
    let pdf_path = "/home/el/Downloads/sample-3.pdf";

    // Check if the path exists before trying to create the window
    if !Path::new(pdf_path).exists() {
         eprintln!("Error: PDF file not found at '{}'", pdf_path);
         return;
    }

    println!("Starting PDF viewer for: {}", pdf_path);
    // Call the function to create and display the window
    create_window(pdf_path);

    // --- Optional: Code for saving the first page as PNG (from original code, slightly improved) ---
    /*
    // Add 'use image::{ImageBuffer, Rgb};' back at the top if using this.
    let page_number = 0;
    println!("Attempting to render PDF page {} to image...", page_number);
    match render_pdf_page_to_image(pdf_path, page_number) {
        Some(fltk_img) => {
            println!("Rendering successful. Saving to output.png...");
            // Get dimensions directly from the FLTK image
            let w = fltk_img.data_w();
            let h = fltk_img.data_h();
            // Get RGB data (might discard alpha, might fail if format mismatch)
            let rgb_pixels = fltk_img.to_rgb_data();

            // Check if data length matches expected RGB (3 bytes/pixel)
            if rgb_pixels.len() == (w * h * 3) as usize {
                // Create an ImageBuffer assuming RGB data
                match ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(w as u32, h as u32, rgb_pixels) {
                    Some(img) => {
                        // Save the image as a PNG file
                        match img.save("output.png") {
                            Ok(_) => println!("Image saved successfully to output.png"),
                            Err(e) => eprintln!("Error saving image: {}", e),
                        }
                    }
                    None => eprintln!("Error: Failed to create image buffer from raw RGB data."),
                }
            } else {
                 eprintln!("Error: Pixel data length ({}) does not match expected RGB size ({}x{}x3={}). Might be RGBA or format issue.",
                    rgb_pixels.len(), w, h, w * h * 3);
                 // If you need RGBA, you might need `fltk_img.data()` and use `ImageBuffer<Rgba<u8>, _>`
            }
        }
        None => eprintln!("Error: Failed to render PDF page for saving."),
    }
    */
}