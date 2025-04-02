// src/app_state.rs
use fltk::frame::Frame;
use pdfium_render::prelude::*;
use std::path::PathBuf;

// Keep the lifetime parameter 'a tied to the Pdfium instance
#[derive(Debug)] // Add Debug for easier inspection if needed
pub struct AppState<'a> {
    pub pdf_path: PathBuf,
    pub doc: PdfDocument<'a>, // Doc borrows from Pdfium instance
    pub current_page: u16, // 0-based index internally
    pub total_pages: u16, // 1-based count for display logic
    pub frame: Frame,     // The FLTK frame widget for displaying the page
}

impl<'a> AppState<'a> {
    // Helper method to get user-friendly current page number (1-based)
    pub fn current_page_display(&self) -> u16 {
        self.current_page + 1
    }

    // Helper method to get the 0-based index of the last page
    pub fn last_page_index(&self) -> u16 {
        self.total_pages.saturating_sub(1)
    }
}