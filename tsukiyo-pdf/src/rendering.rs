use crate::app_state::AppState;
use fltk::{
    enums::ColorDepth,
    image::RgbImage as FltkRgbImage,
    prelude::*,
};
use pdfium_render::{prelude::*, pdfium_render::error::PdfiumError};

/// Renders the current PDF page to fit the frame and updates the FLTK frame widget.
///
/// Calculates the appropriate scale to preserve aspect ratio, renders the page
/// using `pdfium_render`, converts the result to an FLTK image, and displays it.
/// Handles potential errors during rendering or image creation by updating the frame's label.
pub fn render_and_update_frame<'a>(
    state: &mut AppState<'a>,
    frame_w: i32, // Target frame width in pixels
    frame_h: i32, // Target frame height in pixels
) -> Result<(), PdfiumError> { // Propagates errors from the pdfium_render library

    // --- 1. Validate Frame Dimensions ---
    // Skip rendering if the frame area is non-positive.
    if frame_w <= 0 || frame_h <= 0 {
        println!("Skipping render: Invalid frame dimensions (w={}, h={})", frame_w, frame_h);
        state.frame.set_image::<FltkRgbImage>(None); // Clear image
        state.frame.set_label("Resize window");     // Inform user
        state.frame.redraw();                       // Update UI
        return Ok(()); // Not an error, just nothing to render.
    }

    // --- 2. Get PDF Page and Dimensions ---
    // Access the current page using the index from AppState.
    let page = state.doc.pages().get(state.current_page)?; // Propagates PdfiumError if index is invalid

    // Retrieve page dimensions in points (PDF units).
    // Assumes `.width()`/`.height()` return PdfPoints directly.
    let page_w_points = page.width();
    let page_h_points = page.height();

    // Convert dimensions to f32. Assumes PdfPoints has a `.value()` method returning f32.
    // previous version of code didn't work, same deal with current code as well
    // what do we really need here?
    let page_w = page_w_points.value();
    let page_h = page_h_points.value();

    // Validate page dimensions retrieved from the PDF.
    if page_w <= 0.0 || page_h <= 0.0 {
        eprintln!(
            "Skipping render: Invalid page dimensions (w={}, h={})",
            page_w, page_h
        );
        state.frame.set_image::<FltkRgbImage>(None); // Clear image
        state.frame.set_label("Invalid page dimensions");
        state.frame.redraw();                       // Update UI
        return Ok(()); // Invalid data in PDF, but not a library error.
    }

    // --- 3. Calculate Render Scale and Target Size ---
    // Determine the scaling factor to fit the page within the frame while preserving aspect ratio.
    let frame_w_f = frame_w as f32;
    let frame_h_f = frame_h as f32;
    let scale_w = frame_w_f / page_w;
    let scale_h = frame_h_f / page_h;
    let scale = scale_w.min(scale_h); // Use the smaller scale factor to ensure the page fits

    // Calculate the target bitmap size for rendering. Ensure at least 1x1 pixel.
    let render_w = ((page_w * scale).round() as u16).max(1);
    let render_h = ((page_h * scale).round() as u16).max(1);

    // --- 4. Configure and Render Page ---
    // Set up rendering options, primarily target dimensions.
    let render_config = PdfRenderConfig::new()
        .set_target_width(render_w)
        .set_target_height(render_h);

    // Render the page to an in-memory bitmap.
    let bitmap = page.render_with_config(&render_config)?; // Propagates PdfiumError on failure

    // --- 5. Process Rendered Bitmap ---
    // Get dimensions from the rendered bitmap.
    // Assumes `.width()`/`.height()` return PdfPoints.
    let width_points = bitmap.width();
    let height_points = bitmap.height();

    // Convert bitmap dimensions (PdfPoints) to i32 pixels for FLTK.
    // Assumes `f32::from(PdfPoints)` is available.
    // do we need from conversion? 
    let width: i32 = f32::from(width_points).round() as i32;
    let height: i32 = f32::from(height_points).round() as i32;

    // Validate dimensions obtained *from the bitmap* after conversion.
    if width <= 0 || height <= 0 {
        eprintln!(
            "Render Error: Invalid bitmap dimensions after conversion (w={}, h={})",
            width, height
        );
        state.frame.set_image::<FltkRgbImage>(None);
        state.frame.set_label("Render Error (Bitmap Size)");
        state.frame.redraw();
        return Ok(()); // Render produced invalid size bitmap.
    }

    // Get the raw pixel data as RGBA bytes.
    // Assumes `.as_rgba_bytes()` returns a byte slice (`&[u8]`).
    let bytes = bitmap.as_rgba_bytes();

    // Validate that pixel data was actually generated.
    if bytes.is_empty() {
        eprintln!("Render Error: Rendered bitmap has no pixel data.");
        state.frame.set_image::<FltkRgbImage>(None);
        state.frame.set_label("Render Error (Empty Data)");
        state.frame.redraw();
        return Ok(()); // Render produced no data.
    }

    // --- 6. Create FLTK Image and Update Frame ---
    // Attempt to create an FLTK RgbImage from the raw bytes.
    match FltkRgbImage::new(&bytes, width, height, ColorDepth::Rgba8) {
        Ok(fltk_image) => {
            // Success: Update the frame with the new image.
            state.frame.set_image(Some(fltk_image));
            state.frame.set_label(""); // Clear any previous error messages
        }
        Err(e) => {
            // Failure: Log the error and update the frame label.
            eprintln!("FLTK Error: Failed to create image: {:?}", e);
            state.frame.set_image::<FltkRgbImage>(None); // Clear potentially broken image state
            state.frame.set_label("FLTK Image Error");
        }
    }

    // --- 7. Redraw Frame ---
    // Ensure the frame widget is redrawn to show the new image or error label.
    state.frame.redraw();

    Ok(()) // Signal successful completion (or handled non-Pdfium errors)
}