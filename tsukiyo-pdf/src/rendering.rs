use crate::app_state::AppState;
use fltk::{
    enums::ColorDepth,
    image::RgbImage as FltkRgbImage,
    prelude::*,
};

use pdfium_render::{prelude::*, pdfium_render::error::PdfiumError}; // Import PdfiumError

// *** CHANGE: Function now returns Result to allow use of `?` ***
pub fn render_and_update_frame<'a>(
    state: &mut AppState<'a>,
    frame_w: i32,
    frame_h: i32,
) -> Result<(), PdfiumError> { // Return PdfiumError directly for simplicity here

    // --- Initial Frame Size Check ---
    if frame_w <= 0 || frame_h <= 0 {
        println!("Skipping render for zero/negative frame size");
        state.frame.set_image::<FltkRgbImage>(None);
        state.frame.set_label("Resize window");
        state.frame.redraw();
        // Need to return Ok(()) if we exit early without a PdfiumError
        return Ok(());
    }

    // --- Get the Page ---
    // Use `?` to get the page or propagate the error
    let page = state.doc.pages().get(state.current_page)?;

    // --- Get Page Dimensions (Using `?`) ---
    // `?` will return early with the Err variant if page.width() fails
    let page_w_points = page.width()?;
    let page_h_points = page.height()?;

    // Convert page dimensions to f32
    let page_w: f32 = page_w_points.into();
    let page_h: f32 = page_h_points.into();

    // Check page dimensions *after* getting them
    if page_w <= 0.0 || page_h <= 0.0 {
        eprintln!(
            "Skipping render for zero/negative page size (w={}, h={})",
            page_w, page_h
        );
        state.frame.set_label("Invalid page dimensions");
        state.frame.redraw();
        // Return Ok(()) as this isn't a PdfiumError
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

    // --- Get Bitmap Dimensions (Likely Direct Access, No Result) ---
    // *** CORRECTION based on E0308 error messages for bitmap ***
    // The compiler errors (expected i32, found Result) strongly suggest
    // bitmap.width/height *do not* return Result<PdfPoints>, but PdfPoints directly.
    // If they *did* return Result, the error would be "expected PdfPoints".
    let width_points = bitmap.width(); // Assuming this returns PdfPoints directly
    let height_points = bitmap.height(); // Assuming this returns PdfPoints directly

    // Convert bitmap dimensions PdfPoints -> f32 -> i32 for FLTK
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

    // --- Get Pixel Data (Using `?` after correction) ---
    // *** CORRECTION based on E0599: Use as_rgba_bytes() ***
    // as_rgba_bytes returns Result<&[u8], PdfiumError>
    let bytes = bitmap.as_rgba_bytes()?; // Use `?` to handle the Result

    if bytes.is_empty() {
        eprintln!("Rendered bitmap has empty pixel data.");
        state.frame.set_label("Render Error (Empty)");
        state.frame.redraw();
        return Ok(()); // Not a PdfiumError
    }

    // --- Create and Set FLTK Image ---
    // FltkRgbImage::new itself returns a Result<_, FltkError>, but we handle it locally
    match FltkRgbImage::new(bytes, width, height, ColorDepth::Rgba8) {
        Ok(fltk_image) => {
            state.frame.set_image(Some(fltk_image));
            state.frame.set_label(""); // Clear errors
        }
        Err(e) => {
            // This is an FLTK error, not a Pdfium error. Log it.
            eprintln!("Error creating FLTK image: {:?}", e);
            state.frame.set_label("FLTK Image Error");
            // We don't propagate this error type currently, just show it.
        }
    }

    // Crucial: Redraw the frame AFTER setting image or error label
    state.frame.redraw();

    // If we reach here, everything succeeded from Pdfium's perspective
    Ok(())
}