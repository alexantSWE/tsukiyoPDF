// src/error.rs
use std::path::PathBuf;
use thiserror::Error; // Add `thiserror = "1.0"` to Cargo.toml

#[derive(Debug, Error)]
pub enum PdfViewerError {
    #[error("PDF file not found at path: {0}")]
    FileNotFound(PathBuf),

    #[error("Failed to bind to Pdfium system library: {0}")]
    PdfiumBindError(#[from] pdfium_render::prelude::PdfiumError), // Allow conversion

    #[error("Failed to initialize PDFium with bindings: {0}")]
    PdfiumInitError(pdfium_render::prelude::PdfiumError),

    #[error("Failed to load PDF document from '{path}': {source}")]
    PdfLoadError {
        path: PathBuf,
        #[source]
        source: pdfium_render::prelude::PdfiumError,
    },

    #[error("PDF document has no pages")]
    PdfNoPages,

    #[error("Failed to get page count: {0}")]
    PdfPageCountError(pdfium_render::prelude::PdfiumError),

    // You could add more specific errors for rendering, etc. if needed
    #[error("FLTK Error: {0}")]
    FltkError(String), // Catch-all for FLTK issues if needed
}

// Helper type alias for Result using our error type
pub type Result<T> = std::result::Result<T, PdfViewerError>;