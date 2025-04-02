// src/main.rs
use fltk::{app, dialog}; // Import necessary fltk items
use std::path::PathBuf;

// Declare modules
mod app_state;
mod error;
mod rendering;
mod ui;

// Use our custom error type
use error::PdfViewerError;

fn main() {
    /*
    It's good practice to initialize the app object early,
    especially before showing dialogs.
    */
    let _app = app::App::default();

    // --- File Chooser ---
    let pdf_path: PathBuf = match dialog::file_chooser(
        "Open PDF File",       // Title
        "*.pdf",               // Filter
        ".",                   // Default directory (current)
        false,                 // Show hidden files?
    ) {
        Some(path_str) => PathBuf::from(path_str), // Convert String to PathBuf
        None => {
            println!("No file selected. Exiting.");
            dialog::message_default("No PDF file was selected.");
            return; // Exit if the user cancelled
        }
    };

    println!("Attempting to open: {}", pdf_path.display());

    // --- Run the UI ---
    // Pass the selected path to the UI creation function
    if let Err(err) = ui::create_and_run_window(pdf_path) {
        // Handle errors reported by the UI function
        eprintln!("Application Error: {}", err);
        // Show error to user via FLTK dialog
        dialog::alert_default(&format!("Error: {}", err));

        // Optionally exit with an error code
        std::process::exit(1);
    }

    // App exits normally here after window is closed
    println!("Application finished cleanly.");
    // app.quit(); // Usually not needed as app.run() handles termination
}