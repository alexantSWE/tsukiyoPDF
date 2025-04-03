# tsukiyoPDF

A lightweight PDF reader built with Rust, aiming for speed and simplicity.

**(Note: This project is in a very early, experimental stage.)**

## Overview

tsukiyoPDF is an attempt to create a fast and minimal PDF viewer using Rust. The core goals include efficient rendering, low resource usage, and eventually, features like dark mode support.

The name derives from 月夜 (Tsukiyo) - Japanese for "moonlit night" 🌙 - reflecting the aspiration for a polished dark mode experience in the future.

This project serves as a learning exercise for Rust development and exploring native GUI applications outside of common frameworks like Electron or Tauri.

## Current Status

⚠️ **Alpha / Experimental:** tsukiyoPDF is currently under active development and should be considered unstable. It is primarily an educational project at this stage. Expect bugs, crashes, and incomplete features. It is not yet recommended for daily use.

## Technology Stack

*   **Language:** [Rust](https://www.rust-lang.org/)
*   **UI Toolkit:** [FLTK-rs](https://github.com/fltk-rs/fltk-rs) - Chosen for its speed, minimal dependencies, static linking capabilities, and cross-platform consistency.
*   **PDF Backend:** [PDFium](https://pdfium.googlesource.com/pdfium/) via the [pdfium-render](https://crates.io/crates/pdfium-render) crate for rendering PDF pages.
*   **PDF Handling:** [lopdf](https://crates.io/crates/lopdf) is used for interacting with PDF structures when needed.

## Features

### Core Functionality (Work in Progress)

*   Opening and displaying PDF files.
*   Basic page rendering.

### Planned Features / Roadmap

*   **Reliable PDF Rendering:** Improving stability and compatibility across different PDF documents.
*   **Basic Navigation:** Scrolling, zooming, page jumps.
*   **Dark Mode:** A high-priority visual feature.
*   **Performance Optimizations:** Including investigation of hardware acceleration.
*   **Tabbed Interface:** Support for opening multiple documents.
*   **Annotations & Highlighting:** Basic markup capabilities.
*   **Improved Platform Integration:** Including proper icons, especially for Wayland environments.
*   **Robust File Handling:** Ensuring files are closed correctly and resources are managed efficiently.

## Why FLTK?

FLTK was chosen deliberately for several reasons:

*   **Performance:** It's known for being lightweight and fast.
*   **Small Binaries:** Enables the creation of smaller, self-contained applications.
*   **Cross-Platform:** Provides a consistent (though potentially dated-looking) experience across different operating systems.
*   **Simplicity:** Offers a lower-level GUI programming experience, aligning with the learning goals of this project.

## Building from Source

1.  **Install Rust:** If you don't have it, get it from [rustup.rs](https://rustup.rs/).
2.  **Install FLTK Dependencies:** Follow the prerequisites guide for [fltk-rs](https://fltk-rs.github.io/fltk-rs/book/Requirements.html). This usually involves installing C++ build tools and system libraries like Pango, Cairo, and X11 development headers on Linux.
3.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/tsukiyoPDF.git # Replace with actual repo URL
    cd tsukiyoPDF
    ```
4.  **Build:**
    ```bash
    cargo build --release
    ```
5.  **Run:**
    ```bash
    cargo run --release
    # Or find the executable in ./target/release/
    ```

## Contributing

Contributions are welcome, especially given the early stage of the project!

If you'd like to help:

1.  **Fork** the repository.
2.  Create a **new branch** for your feature or bug fix (`git checkout -b feature/my-new-feature` or `fix/specific-bug`).
3.  Make your **changes**.
4.  **Commit** your changes with clear messages.
5.  Push your branch and open a **Pull Request** against the main branch of the original repository. Please describe your changes clearly in the PR.
6.  You can also **open an Issue** to report bugs, suggest features, or ask questions.

As the primary developer is also learning, feedback and collaborative efforts are highly appreciated.

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
