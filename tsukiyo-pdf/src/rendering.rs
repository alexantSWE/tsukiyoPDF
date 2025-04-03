use crate::app_state::AppState;
use fltk::{
    enums::ColorDepth,
    image::RgbImage as FltkRgbImage,
    prelude::*,
};

// Assuming PdfPoints might need to be explicitly imported if not in prelude
// e.g. use pdfium_render::prelude::PdfPoints;
use pdfium_render::{prelude::*, pdfium_render::error::PdfiumError};

pub fn render_and_update_frame<'a>(
    state: &mut AppState<'a>,
    frame_w: i32,
    frame_h: i32,
) -> Result<(), PdfiumError> { // Return PdfiumError

    // --- Initial Frame Size Check ---
    if frame_w <= 0 || frame_h <= 0 {
        println!("Skipping render for zero/negative frame size");
        state.frame.set_image::<FltkRgbImage>(None);
        state.frame.set_label("Resize window");
        state.frame.redraw();
        return Ok(());
    }

    // --- Get the Page ---
    // Assuming pages().get() returns Result<Page, PdfiumError>
    let page = state.doc.pages().get(state.current_page)?;

    // --- Get Page Dimensions (Using `?`) ---
    // Assuming page.width/height return Result<PdfPoints, PdfiumError>
    let page_w_points = page.width();
    let page_h_points = page.height();

    // Convert page dimensions to f32
    // Assuming PdfPoints implements Into<f32> or From<PdfPoints> for f32
    let page_w: f32 = page_w_points.
    let page_h: f32 = page_h_points.value():

    // Check page dimensions *after* getting them
    if page_w <= 0.0 || page_h <= 0.0 {
        eprintln!(
            "Skipping render for zero/negative page size (w={}, h={})",
            page_w, page_h
        );
        state.frame.set_label("Invalid page dimensions");
        state.frame.redraw();
        return Ok(());
    }

    // --- Aspect Ratio Calculation ---
    let frame_w_f = frame_w as f32;
    let frame_h_f = frame_h as f32;
    let scale_w = frame_w_f / page_w;
    let scale_h = frame_h_f / page_h;
    let scale = scale_w.min(scale_h);
    let render_w = ((page_w * scale).round() as u16).max(1);
    let render_h = ((page_h * scale).round() as u16).max(1);

    // --- Rendering Configuration ---
    let render_config = PdfRenderConfig::new()
        .set_target_width(render_w)
        .set_target_height(render_h);

    // --- Render the Page (Using `?`) ---
    let bitmap = page.render_with_config(&render_config)?;

    // --- Get Bitmap Dimensions (Using `?`) ---
    // *** CHANGE: Added ? based on revised analysis ***
    // Assuming bitmap.width/height also return Result<PdfPoints, PdfiumError>
    let width_points = bitmap.width();  // ADDED ?
    let height_points = bitmap.height(); // ADDED ?

    // Convert bitmap dimensions PdfPoints -> f32 -> i32 for FLTK
    // This should now work correctly as width_points/height_points are PdfPoints
    // (Still assumes PdfPoints implements Into<f32> or From<PdfPoints> for f32)
    let width: i32 = f32::from(width_points).round() as i32;
    let height: i32 = f32::from(height_points).round() as i32;

    // Check bitmap dimensions after conversion
    if width <= 0 || height <= 0 {
        eprintln!(
            "Bitmap dimension is zero or negative after conversion (w={}, h={})",
            width, height
        );
        state.frame.set_label("Render Error (Conv Size)");
        state.frame.redraw();
        return Ok(()); // Not a PdfiumError
    }

    // --- Get Pixel Data (Using `?`) ---
    let bytes = bitmap.as_rgba_bytes(); // This seemed correct before

    if bytes.is_empty() {
        eprintln!("Rendered bitmap has empty pixel data.");
        state.frame.set_label("Render Error (Empty)");
        state.frame.redraw();
        return Ok(()); // Not a PdfiumError
    }

    // --- Create and Set FLTK Image ---
    match FltkRgbImage::new(bytes, width, height, ColorDepth::Rgba8) {
        Ok(fltk_image) => {
            state.frame.set_image(Some(fltk_image));
            state.frame.set_label(""); // Clear errors
        }
        Err(e) => {
            eprintln!("Error creating FLTK image: {:?}", e);
            state.frame.set_label("FLTK Image Error");
            state.frame.redraw();

        }
    }

    // Crucial: Redraw the frame
    
    Ok(())
}