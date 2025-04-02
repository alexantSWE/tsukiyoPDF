use fltk::{
    app,
    enums::{ColorDepth, Event, Key},
    frame::Frame,
    image::RgbImage as FltkRgbImage,
    prelude::*,
    window::Window,
};
use pdfium_render::prelude::*;
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    // No longer need OnceLock for Pdfium itself
};

// --- Application State ---
// Add a lifetime parameter 'a tied to the Pdfium instance
struct AppState<'a> { // <--- Added lifetime 'a
    pdf_path: PathBuf,
    doc: PdfDocument<'a>, // <--- Doc now borrows from Pdfium instance with lifetime 'a
    current_page: u16,
    total_pages: u16,
    frame: Frame,
}

// --- Rendering Logic ---
// Function signature now requires the lifetime 'a for AppState
fn render_and_update_frame<'a>(state: &mut AppState<'a>, frame_w: i32, frame_h: i32) {
    if frame_w <= 0 || frame_h <= 0 {
        println!("Skipping render for zero/negative frame size");
        // Optionally clear the frame or show a placeholder
        state.frame.set_image::<FltkRgbImage>(None); // Clear previous image
        state.frame.set_label("Resize window");
        state.frame.redraw();
        return;
    }

    // Get the page (handle potential errors)
    let page_result = state.doc.pages().get(state.current_page);
    let page = match page_result {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "Error getting page index {}: {:?}", // 0-based index for error msg
                state.current_page,
                e
            );
            state.frame.set_label(&format!("Error loading page {}", state.current_page + 1));
            state.frame.redraw();
            return;
        }
    };

    fn render_and_update_frame<'a>(state: &mut AppState<'a>, frame_w: i32, frame_h: i32) {
        // ... (previous checks for frame_w, frame_h) ...
        // ... (getting the page) ...
    
        // --- Get Page Dimensions CORRECTLY ---
        let page_w_result = page.width();
        let page_h_result = page.height();
    
        // Handle potential errors when getting dimensions
        let page_w_points = match page_w_result {
            Ok(w) => w, // Corrected: Assign w directly if Ok
            Err(e) => {
                // Corrected: Provide page number (user-friendly) and the error e
                eprintln!(
                    "Error getting page width for page {}: {:?}",
                    state.current_page + 1, // User-friendly 1-based index
                    e
                );
                // Corrected: Use correct syntax for set_label
                state.frame.set_label("Error: Page Width");
                state.frame.redraw();
                return; // Return early on error
            }
        }; // Semicolon needed here for the let binding
    
        let page_h_points = match page_h_result {
            Ok(h) => h, // Corrected: Assign h directly if Ok
            Err(e) => {
                // Corrected: Provide page number (user-friendly) and the error e
                eprintln!(
                    "Error getting page height for page {}: {:?}",
                    state.current_page + 1, // User-friendly 1-based index
                    e
                );
                // Corrected: Use correct syntax for set_label
                state.frame.set_label("Error: Page Height");
                state.frame.redraw();
                return; // Return early on error
            }
        }; // Semicolon needed here for the let binding
    
        // Extract f32 values from PdfPoints using .into()
        let page_w: f32 = page_w_points.into();
        let page_h: f32 = page_h_points.into();

    // Check for zero/negative dimensions *after* getting them
    if page_w <= 0.0 || page_h <= 0.0 {
        eprintln!("Skipping render for zero/negative page size (w={}, h={})", page_w, page_h);
        state.frame.set_label("Invalid page dimensions");
        state.frame.redraw();
        return;
    }
    // --- End of Dimension Fixes ---


    // --- Aspect Ratio Calculation (uses page_w, page_h as f32) ---
    let frame_w_f = frame_w as f32;
    let frame_h_f = frame_h as f32;

    // Calculate scaling factors
    let scale_w = frame_w_f / page_w;
    let scale_h = frame_h_f / page_h;
    // Use the smaller scale factor to fit without distortion
    let scale = scale_w.min(scale_h);

    // Calculate target render dimensions (in pixels)
    // Ensure dimensions are at least 1 pixel if scaling results in zero
    let render_w = ((page_w * scale).round() as u16).max(1);
    let render_h = ((page_h * scale).round() as u16).max(1);

    // Configure rendering for the calculated size
    let render_config = PdfRenderConfig::new()
        .set_target_width(render_w)
        .set_target_height(render_h)
        // Generally, Pdfium's default rasterizer provides good anti-aliasing
        // when rendering to the target resolution.
        // You usually don't need explicit AA flags unless troubleshooting.
        ;

    // Render the page
    match page.render_with_config(&render_config) {
        Ok(bitmap) => { // <--- Inside this block, 'bitmap' IS of type PdfBitmap

             // 1. Get the Result for width
             let width_result = bitmap.width(); // Assuming this returns Result<PdfPoints, ...>

             // 2. Match the Result for width
             let width_points = match width_result {
                 Ok(w) => w, // Extract PdfPoints if Ok
                 Err(e) => {
                     eprintln!("Error getting bitmap width after rendering page {}: {:?}", state.current_page + 1, e);
                     state.frame.set_label("Error: Bitmap Width");
                     state.frame.redraw();
                     return; // Return early
                 }
             };
 
             // 3. Get the Result for height
             let height_result = bitmap.height(); // Assuming this returns Result<PdfPoints, ...>
 
             // 4. Match the Result for height
             let height_points = match height_result {
                 Ok(h) => h, // Extract PdfPoints if Ok
                 Err(e) => {
                     eprintln!("Error getting bitmap height after rendering page {}: {:?}", state.current_page + 1, e);
                     state.frame.set_label("Error: Bitmap Height");
                     state.frame.redraw();
                     return; // Return early
                 }
             };
 
             // 5. Convert PdfPoints to f32
             let width_f32: f32 = width_points.into();
             let height_f32: f32 = height_points.into();
 
             // 6. Convert f32 to i32 for FltkRgbImage (rounding is usually best)
             let width: i32 = width_f32.round() as i32;
             let height: i32 = height_f32.round() as i32;
             // --- End of Bitmap Dimension Handling ---
 
 
             // Ensure dimensions are positive after potential rounding
              if width <= 0 || height <= 0 {
                  eprintln!("Bitmap dimension is zero or negative after conversion (w={}, h={})", width, height);
                  state.frame.set_label("Render Error (Conv Size)");
                  state.frame.redraw();
                  return;
              }
 
             // Get pixel data (into_rgba_bytes consumes bitmap)
             // NOTE: If bitmap.width/height really return Result<PdfPoints>,
             // calling bitmap.into_rgba_bytes() *after* might be problematic if
             // into_rgba_bytes depends on width/height internally without re-checking.
             // This API seems increasingly strange if this is the case.
             let bytes = bitmap.into_rgba_bytes();
 
              if bytes.is_empty() {
                  eprintln!("Rendered bitmap has empty pixel data.");
                  state.frame.set_label("Render Error (Empty)");
                  state.frame.redraw();
                  return;
              }
 
              // Create FLTK image using the final i32 width/height
             match FltkRgbImage::new(&bytes, width, height, ColorDepth::Rgba8) {
                 Ok(fltk_image) => {
                     state.frame.set_image(Some(fltk_image));
                     state.frame.set_label("");
                 }
                 Err(e) => {
                     eprintln!("Error creating FLTK image: {:?}", e);
                     state.frame.set_label("FLTK Image Error");
                 }
             }
         }
         Err(e) => {
             eprintln!(
                 "Error rendering page {} with config {:?}: {:?}",
                 state.current_page + 1,
                 render_config,
                 e
             );
             state.frame.set_label("PDF Render Error");
         }
     }
     // Crucial: Redraw the frame after attempting to set the image or label
     state.frame.redraw();
    }


// --- Main Window Creation ---
pub fn create_window(pdf_path_str: &str) {
    let pdf_path = PathBuf::from(pdf_path_str);
    if !pdf_path.exists() {
        eprintln!("Error: PDF file not found at '{}'", pdf_path_str);
        // Use FLTK dialog for user feedback if possible before exiting
        let _ = fltk::dialog::alert_default(&format!("Error: PDF file not found at\n{}", pdf_path_str));
        return;
    }

    // --- Initialize Pdfium Here ---
    // Pdfium instance lives for the duration of `create_window`
    let pdfium = match Pdfium::bind_to_system_library() {
         Ok(bindings) => Pdfium::new(bindings),
         Err(e) => {
             eprintln!("FATAL: Failed to bind to Pdfium system library: {:?}", e);
             eprintln!("Please ensure the Pdfium library is installed and accessible.");
             let _ = fltk::dialog::alert_default(&format!("Failed to initialize PDF engine: {:?}\nPlease ensure Pdfium library is installed.", e));
             return;
         }
    }; // pdfium instance created here

    // --- Load the document using the local Pdfium instance ---
    // The resulting 'doc' borrows from 'pdfium'
    let doc = match pdfium.load_pdf_from_file(&pdf_path, None) {
        Ok(d) => d, // 'd' has lifetime tied to 'pdfium'
        Err(e) => {
            eprintln!("Error loading PDF '{}': {:?}", pdf_path_str, e);
            let _ = fltk::dialog::alert_default(&format!("Failed to load PDF:\n{}\nError: {}", pdf_path.display(), e));
            return;
        }
    }; // 'doc' created here

    // --- Get Page Count ---
    let total_pages = match doc.pages().len() {
         Ok(len) if len > 0 => len, // Proceed only if pages exist
         Ok(_) => { // len == 0
             println!("PDF has no pages.");
             let _ = fltk::dialog::alert_default("The selected PDF file contains no pages.");
             return;
         },
        Err(e) => {
             eprintln!("Error getting page count: {:?}", e);
             let _ = fltk::dialog::alert_default(&format!("Failed get page count: {}", e));
             return;
        }
    };


    let app = app::App::default().with_scheme(app::Scheme::Gtk); // Or try Gleam/Plastic
    let mut wind = Window::new(100, 100, 800, 600, "Tsukiyo PDF Viewer"); // Initial size

    // Frame starts filling the window
    let mut frame = Frame::new(0, 0, wind.w(), wind.h(), "");
    frame.set_frame(fltk::enums::FrameType::NoBox); // Avoid frame drawing over image edge
    wind.end();

    // Make the window resizable, with the frame being the resizable element
    wind.make_resizable(true);
    wind.resizable(&frame); // The frame will be resized by the window manager

    // --- State Management ---
    // Create AppState. 'doc' is moved in. The state now borrows from 'pdfium'
    // via the 'doc' field, implicitly getting the lifetime 'a.
    let app_state = Rc::new(RefCell::new(AppState { // Lifetime 'a is inferred here
        pdf_path,
        doc, // Move the document into the state
        current_page: 0, // Start at first page (0-based index)
        total_pages,
        frame, // Move the frame widget into the state
    }));

    // --- Initial Render ---
    { // Create a scope to borrow mutably then release immediately
        let mut state_mut = app_state.borrow_mut();
        let initial_w = state_mut.frame.w();
        let initial_h = state_mut.frame.h();
        // Perform initial render
        render_and_update_frame(&mut state_mut, initial_w, initial_h);
        // Update window title initially
        wind.set_label(&format!("Page {}/{} - Tsukiyo PDF Viewer", state_mut.current_page + 1, state_mut.total_pages));
    } // state_mut borrow ends here


    // --- Resize Callback ---
    // Attached to the window, triggered when the window size changes
    let state_for_resize = app_state.clone();
    wind.set_callback(move |w| { // 'w' is the window widget
        // When the window resizes, the 'frame' inside our state should have been resized too
        // because we set `wind.resizable(&frame)`
        let mut state = state_for_resize.borrow_mut();
        let frame_w = state.frame.w(); // Get the *new* size of the frame
        let frame_h = state.frame.h();

        // Render only if the size is valid
        if frame_w > 0 && frame_h > 0 {
             // println!("Window resized, rendering for frame size: {}x{}", frame_w, frame_h); // Debug
             render_and_update_frame(&mut state, frame_w, frame_h);
        } else {
            // This might happen if the window is minimized or has an invalid size reported
            println!("Window resize resulted in zero frame dimension, skipping render.");
            state.frame.set_image::<FltkRgbImage>(None); // Clear image
            state.frame.set_label("Invalid size");
            state.frame.redraw();
        }
        // We don't need app::redraw() usually, the render function redraws the frame.
    });


    // --- Keypress Handler ---
    // Attached to the window to capture key events globally for the app
    let state_for_keys = app_state.clone();
    // Need access to window to update title
    let mut wind_clone = wind.clone();
    wind.handle(move |_, ev| { // Widget arg is the window ('_'), ev is the event
        match ev {
            Event::KeyDown => {
                let mut state = state_for_keys.borrow_mut(); // Mutable access to state
                let mut page_changed = false;
                let current_page = state.current_page; // Copy to avoid borrow conflicts
                let total_pages = state.total_pages;

                match app::event_key() {
                    // Navigation Keys
                    Key::PageDown | Key::Right | Key::Down => {
                        if current_page < total_pages.saturating_sub(1) {
                            state.current_page += 1;
                            page_changed = true;
                        } else {
                             println!("Already on last page ({})", total_pages); // Info
                             // Optionally flash background or something?
                             return true; // Consume event even if no change
                        }
                    }
                    Key::PageUp | Key::Left | Key::Up => {
                        if current_page > 0 {
                            state.current_page -= 1;
                            page_changed = true;
                        } else {
                             println!("Already on first page (1)"); // Info
                             return true; // Consume event
                        }
                    }
                    // Go to Start/End
                    Key::Home => {
                         if current_page != 0 {
                              state.current_page = 0;
                              page_changed = true;
                         } else {
                              return true; // Consume event
                         }
                    }
                    Key::End => {
                         let last_page = total_pages.saturating_sub(1);
                         if current_page != last_page {
                              state.current_page = last_page;
                              page_changed = true;
                         } else {
                              return true; // Consume event
                         }
                    }
                    // Add more keybinds here if needed (e.g., zoom, search)

                    _ => return false, // Key not handled by us, let FLTK process (e.g., Alt+F4)
                }

                // If the page changed, re-render and update title
                if page_changed {
                    println!("Navigating to Page {}", state.current_page + 1); // User feedback
                    let frame_w = state.frame.w(); // Get current frame size
                    let frame_h = state.frame.h();
                     // Render only if frame size is valid
                     if frame_w > 0 && frame_h > 0 {
                         render_and_update_frame(&mut state, frame_w, frame_h);
                         // Update window title
                         wind_clone.set_label(&format!("Page {}/{} - Tsukiyo PDF Viewer", state.current_page + 1, state.total_pages));
                     } else {
                         println!("Frame size invalid after page change, skipping render.");
                         // Still update the page number logic, but don't render visually
                          state.frame.set_image::<FltkRgbImage>(None);
                          state.frame.set_label("Invalid size");
                          state.frame.redraw();
                     }
                    return true; // Event was handled
                } else {
                     // This branch might not be reached if we return true above,
                     // but keep for clarity if logic changes.
                     return false;
                }
            }
            _ => false, // Event type not handled (e.g., MouseMove, Focus)
        }
    });

    // Show the window
    wind.show();

    // --- Event Loop ---
    // IMPORTANT: 'pdfium' and 'doc' (inside app_state) must live until app.run() finishes.
    // Since they are created in this function scope, and app.run() blocks here,
    // they will live long enough.
    app.run().unwrap();

    // 'app_state', 'doc' (within app_state), and 'pdfium' are dropped here after app.run() returns.

} // <-- pdfium instance is dropped here, AppState goes out of scope

// --- Main Function ---
fn main() {
    // Define the path to the PDF file
    // Consider using command-line arguments for flexibility:
    let args: Vec<String> = std::env::args().collect();
    let pdf_path = if args.len() > 1 {
        &args[1]
    } else {
        // Default path if no argument is provided
        // CHANGE THIS to a PDF that exists on your system for testing
        "/home/el/Downloads/sample-3.pdf"
        // Or show a file chooser dialog?
        // e.g., using fltk::dialog::file_chooser(...)
    };

    println!("Starting PDF viewer for: {}", pdf_path);
    create_window(pdf_path); // Call the function that sets up and runs the app
}