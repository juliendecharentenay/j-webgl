use super::{Result};

pub trait Camera {
  fn as_f32_array(&self) -> Result<&[f32]>;
}

pub trait Renderable {
  fn render(
    &self, 
    context: &web_sys::WebGl2RenderingContext,
    camera: &[f32],
  ) -> Result<()>;
}
