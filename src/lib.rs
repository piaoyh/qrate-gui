// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////



//! # Qrate-gui
//! 
//! qrate-gui is a graphical interface library that leverages Iced for its UI
//! components and qrate for managing question banks and student data.
//! 
//! It enables developers to build applications where users can configure
//! examination parameters and seamlessly receive generated results.

rust_i18n::i18n!("locales", fallback = "en-US");


// All Slint related code has been removed. This library will be integrated into the Iced application.

/// The core logic and state management for the Qrate-GUI application.
mod control_tower;

/// Re-exports the main application components for external use.
pub use control_tower::{ ControlTower, Message };