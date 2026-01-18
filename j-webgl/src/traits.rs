use super::{Result};

/// Trait to implement for camera/view matrix providers
///
/// This trait allows any type to provide a Model-View-Projection (MVP) matrix
/// for rendering operations. The matrix should be provided as a flat array of
/// 16 `f32` values representing a 4x4 matrix.
///
pub trait Camera {
  /// Output camera Model-View-Projection matrix
  ///
  /// Returns a reference to a slice of 16 `f32` values representing a 4x4 matrix.
  /// The matrix format should be column-major (OpenGL/WebGL standard).
  ///
  /// # Errors
  ///
  /// Returns an error if the matrix cannot be generated or accessed.
  ///
  /// TODO: Check if format is column or row based
  fn as_f32_array(&self) -> Result<&[f32]>;
}

/// Trait to be implemented for struct that can be rendered
///
/// Any type implementing this trait can be added to a `Renderer` and will be
/// drawn during the render pass. The implementation is responsible for setting
/// up the necessary WebGL state, binding buffers, setting uniforms, and drawing.
pub trait Renderable {
  /// Trigger rendering with a given context and camera Model-View-Projection matrix
  ///
  /// This method is called by the `Renderer` for each renderable object during
  /// a render pass. The implementation should:
  ///
  /// 1. Set up the shader program
  /// 2. Bind necessary buffers (VAO, VBO, EBO, etc.)
  /// 3. Set uniforms (including the camera MVP matrix)
  /// 4. Issue draw calls
  ///
  /// # Parameters
  ///
  /// - `context`: The WebGL 2.0 rendering context
  /// - `camera_mvp`: A slice of 16 `f32` values representing the camera's MVP matrix
  ///
  /// # Errors
  ///
  /// Returns an error if rendering fails (e.g., shader compilation, buffer binding, etc.)
  fn render(
    &self, 
    context: &web_sys::WebGl2RenderingContext,
    camera_mvp: &[f32],
  ) -> Result<()>;
}
