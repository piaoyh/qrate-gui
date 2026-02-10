# qrate-gui: A Slint-based Graphical Interface for Examination Generation

`qrate-gui` is a high-level Rust crate designed to build graphical applications for effortless examination generation. Built using the **Slint UI framework** and the robust **`qrate` core engine**, this crate provides the necessary components to transform complex randomization logic into an intuitive, modern desktop experience.

## Overview

While the `qrate` crate handles the "brains" of exam generation (shuffling, variety, and logic), `qrate-gui` provides the "body" and "eyes." It is specifically tailored for developers building with **Slint**, allowing for smooth, reactive, and cross-platform user interfaces that educators can use with ease.

## Why qrate-gui with Slint?

The combination of `qrate` logic and `Slint` UI enables the development of professional tools where:

* **Modern Visuals:** Leverage Slint's declarative syntax to create sleek, high-performance interfaces.
* **Visual Management:** Teachers can preview question sets, adjust layouts, and manage student rosters before final output.
* **Reactive Configuration:** Real-time feedback when adjusting randomization parameters via sliders or input fields.
* **Cross-Platform Accessibility:** Easily compile your exam generator for Windows, macOS, or Linux.

## Key Features (Powered by qrate & Slint)

Any application built with `qrate-gui` inherits sophisticated anti-cheating mechanisms with a polished UI:

* **Dynamic Question Selection:** Automatically picks different subsets of questions for each student.
* **Positional Randomization:** Ensures the same question appears at different indices across papers.
* **Option Shuffling:** Permutes multiple-choice answers uniquely for every single exam set.
* **Slint Integration:** Pre-defined data structures and callbacks designed to work seamlessly with Slint's property system.

## Background

`qrate-gui` was born from the need to make the power of the `qrate` engine available to the wider educational community. By decoupling the graphical interface from the core generation logic, we have created a modular system that is both developer-friendly and end-user accessible.

## Purpose

The primary purpose of `qrate-gui` is to **bridge the gap between complex randomization algorithms and the classroom.** By utilizing Slint, we ensure that the resulting tools are not only powerful but also aesthetically pleasing and easy to navigate for educators who may not be tech-savvy.

## Quick Start for Developers

To use `qrate-gui` in your Slint project, add it to your `Cargo.toml`:

```toml
[dependencies]
qrate = "0.4"        # The core engine
qrate-gui = "0.1"    # This crate
slint = "1.15"       # UI Framework
```

### Basic Integration Concept

`qrate-gui` simplifies the connection between your Slint UI and the generation logic. Below is a conceptual example of how to link a Slint callback to the engine:

```rust
slint::slint! {
    export component AppWindow inherits Window {
        callback generate_exams();
        // Your Slint UI code here...
    }
}

fn main() {
    let ui = AppWindow::new().unwrap();
    
    ui.on_generate_exams({
        let ui_handle = ui.as_weak();
        move || {
            // qrate-gui handles the translation between 
            // UI input and the qrate engine logic
        }
    });

    ui.run().unwrap();
}
```

## Caution: Font Requirements

Since `qrate-gui` handles PDF generation with specific styling, your application's working directory must contain a `./fonts` subdirectory. The following files are required for proper rendering:

* `font-Regular.ttf`
* `font-Italic.ttf`
* `font-Bold.ttf`
* `font-BoldItalic.ttf`

---
*Developed through a synergy of Human Intelligence (HI) and Artificial Intelligence (AI). Every line of code reflects both AI-driven efficiency and rigorous human craftsmanship.*