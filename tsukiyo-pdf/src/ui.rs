// src/ui.rs
use crate::app_state::AppState;
use crate::error::{PdfViewerError, Result}; // Use our custom error/result
use crate::rendering::render_and_update_frame;
use fltk::{
    app,
    enums::{Event, Key, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};
use pdfium_render::prelude::*;
use std::{cell::RefCell, path::{Path, PathBuf}, rc::Rc};

// --- Main Window Creation ---
// Now returns a Result to propagate errors
pub fn create_and_run_window(pdf_path: PathBuf) -> Result<()> {
    if !pdf_path.exists() {
        return Err(PdfViewerError::FileNotFound(pdf_path));
    }

    // --- Initialize Pdfium ---
    // Using Pdfium::bind_to_library() is often more robust if you bundle pdfium.dll/so/dylib
    // Using Pdfium::bind_to_system_library() requires it to be installed system-wide.
    let pdfium_bindings = Pdfium::bind_to_system_library()
                            .map_err(PdfViewerError::PdfiumBindError)?;
    let pdfium = Pdfium::new(pdfium_bindings);
       // .map_err(|e| PdfViewerError::PdfiumInitError(e))?; // Pdfium::new doesn't return Result currently

    // --- Load Document ---
    // The lifetime of 'doc' will be tied to 'pdfium' which lives in this function scope
    let doc = pdfium
        .load_pdf_from_file(&pdf_path, None)
        .map_err(|source| PdfViewerError::PdfLoadError {
            path: pdf_path.clone(),
            source,
        })?;

    // --- Get Page Count ---
    let total_pages = match doc.pages().len() {
        Ok(len) if len > 0 => len as u16, // Cast to u16
        Ok(_) => return Err(PdfViewerError::PdfNoPages),
        Err(e) => return Err(PdfViewerError::PdfPageCountError(e)),
    };

    // --- FLTK App and Window Setup ---
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 800, 600, "Tsukiyo PDF Viewer");
    let mut frame = Frame::new(0, 0, wind.w(), wind.h(), "");
    frame.set_frame(FrameType::NoBox);
    wind.end();
    wind.make_resizable(true);
    wind.resizable(&frame); // Frame is the resizable element

    // --- State Management ---
    // AppState borrows 'doc', which borrows 'pdfium'. Lifetime 'a is inferred.
    let app_state = Rc::new(RefCell::new(AppState {
        pdf_path, // Move path in
        doc,      // Move doc in
        current_page: 0, // Start at 0-based index
        total_pages,
        frame,    // Move frame in
    }));

    // --- Initial Render ---
    {
        let mut state_mut = app_state.borrow_mut();
        let initial_w = state_mut.frame.w();
        let initial_h = state_mut.frame.h();
        render_and_update_frame(&mut state_mut, initial_w, initial_h);
        wind.set_label(&format!(
            "Page {}/{} - Tsukiyo PDF Viewer",
            state_mut.current_page_display(),
            state_mut.total_pages
        ));
    }

    // --- Resize Callback ---
    let state_for_resize = app_state.clone();
    wind.set_callback(move |_| { // Use |_| for unused window widget arg
        let mut state = state_for_resize.borrow_mut();
        let frame_w = state.frame.w();
        let frame_h = state.frame.h();
        render_and_update_frame(&mut state, frame_w, frame_h); // Render function handles bad sizes
    });

    // --- Keypress Handler ---
    let state_for_keys = app_state.clone();
    let mut wind_clone = wind.clone(); // Clone window for title updates
    wind.handle(move |_, ev| {
        match ev {
            Event::KeyDown => {
                let mut state = state_for_keys.borrow_mut();
                let mut page_changed = false;
                let current_idx = state.current_page;
                let last_idx = state.last_page_index();

                match app::event_key() {
                    Key::PageDown | Key::Right | Key::Down => {
                        if current_idx < last_idx {
                            state.current_page += 1;
                            page_changed = true;
                        } else {
                            println!("Already on last page ({})", state.total_pages);
                        }
                    }
                    Key::PageUp | Key::Left | Key::Up => {
                        if current_idx > 0 {
                            state.current_page -= 1;
                            page_changed = true;
                        } else {
                            println!("Already on first page (1)");
                        }
                    }
                    Key::Home => {
                        if current_idx != 0 {
                            state.current_page = 0;
                            page_changed = true;
                        }
                    }
                    Key::End => {
                        if current_idx != last_idx {
                            state.current_page = last_idx;
                            page_changed = true;
                        }
                    }
                    _ => return false, // Key not handled by us
                }

                if page_changed {
                    println!("Navigating to Page {}", state.current_page_display());
                    let frame_w = state.frame.w();
                    let frame_h = state.frame.h();
                    render_and_update_frame(&mut state, frame_w, frame_h);
                    wind_clone.set_label(&format!(
                        "Page {}/{} - Tsukiyo PDF Viewer",
                        state.current_page_display(),
                        state.total_pages
                    ));
                    return true; // Event handled
                } else {
                    // Even if page didn't change (e.g., already at end),
                    // we consumed the key event.
                    return true;
                }
            }
            _ => false, // Event type not handled
        }
    });

    // --- Show Window and Run App ---
    wind.show();

    // The event loop. 'pdfium' and 'doc' (in app_state) live until this function returns.
    // The Rc<RefCell<AppState>> ensures state is accessible in callbacks.
    app.run().map_err(|e| PdfViewerError::FltkError(e.to_string()))?; // Handle potential FLTK error

    Ok(()) // Indicate success
} // pdfium, doc, app_state, etc. are dropped here