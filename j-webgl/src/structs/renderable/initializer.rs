use super::*;

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
pub struct Initializer {}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Initializer {
  pub fn default() -> Self { Initializer {} }
}

impl traits::Renderable for Initializer {
  fn render(&self,
    context: &web_sys::WebGl2RenderingContext,
    _camera: &[f32],
  ) -> Result<()>
  {
    // Clear the canvas
    context.clear_color(0.1, 0.1, 0.1, 1.0);
    context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT | web_sys::WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    // Enable depth testing
    context.enable(web_sys::WebGl2RenderingContext::DEPTH_TEST);
    context.enable(web_sys::WebGl2RenderingContext::CULL_FACE);
    
    Ok(())
  }
}
