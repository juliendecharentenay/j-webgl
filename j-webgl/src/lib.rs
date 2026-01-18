//! # j-webgl
//!
//! A Rust wrapper around WebGL 2.0 designed to facilitate code reuse and simplify
//! WebGL rendering operations in WebAssembly applications.
//!
//! ## Overview
//!
//! This crate provides a high-level abstraction over WebGL 2.0, making it easier to
//! create and manage WebGL rendering contexts, compile shaders, and render objects
//! in Rust/WASM applications.
//!
//! ## Features
//!
//! - **Renderer**: High-level renderer that manages WebGL context and renderable objects
//! - **Traits**: `Renderable` and `Camera` traits for flexible rendering architecture
//! - **Shader Utilities**: Helper functions for compiling and linking shader programs
//! - **Reusable Components**: Pre-built shaders and structs for common use cases
//! - **Algebra Types**: 3D math types (`Point3`, `Vector3`, `Matrix4`) for graphics operations
//! - **Macro Support**: `make_renderer!` macro for generating specialized renderer classes
//!
//! ## Quick Start
//!
//! TODO
//!
//! ## Architecture
//!
//! The crate follows a trait-based architecture:
//!
//! - **`Renderable`**: Implement this trait for any object that can be rendered
//! - **`Camera`**: Implement this trait for camera/view matrices
//! - **`Renderer`**: Manages the WebGL context and orchestrates rendering of `Renderable` objects
//!
//! ## Examples
//!
//! See the `examples/` directory for complete working examples, including Vue.js integration.


/// Macro to generate specialist renderer class
///
/// This macro helps create specialized renderer implementations with custom
/// rendering logic. See the `j-webgl-macro-make-renderer` crate for details.p
pub use j_webgl_macro_make_renderer::make_renderer;

mod error;
mod algebra;

/// Library of utility functions for WebGL operations
///
/// This module provides helper functions for common WebGL tasks, including
/// [`compile_program`](utils::compile_program) to compile shaders and link a program.
pub mod utils;

/// Shaders available for reuse
///
/// This module contains pre-written vertex and fragment shaders that can be
/// used directly or as templates for custom shaders.
pub mod shaders;

pub mod programs;

mod traits;
pub use traits::{Renderable, Camera};

pub use error::{Error, Result};

mod renderer;
pub use renderer::{Renderer};

/// Re-usable struct implementing library traits
///
/// This module contains concrete implementations of the library's traits,
/// such as basic camera and renderable structs that can be used directly
/// or extended for custom behavior.
pub mod structs;

/// Re-export libraries/external crates for re-use
///
/// This module re-exports commonly used external crates to simplify imports
/// in dependent code.
pub mod exports {
  pub use web_sys;
  pub use wasm_bindgen;
}
