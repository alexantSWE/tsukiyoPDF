use lopdf::Document;
use log::{info, error};
use std::fs;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

mod window;


fn main() {
    // Initialize logger
    env_logger::init();
    window::create_window();

    let file_path = "/home/el/Downloads/sample-4.pdf";

    info!("Attempting to open PDF file: {}", file_path);

    // Check if file exists
    if !fs::metadata(file_path).is_ok() {
        error!("File does not exist: {}", file_path);
        return;
    }

    match Document::load(file_path) {
        Ok(doc) => {
            info!("Successfully opened PDF.");
            info!("Number of pages: {}", doc.get_pages().len());
        }
        Err(e) => {
            error!("Failed to open PDF: {}", e);
        }
    }
}

