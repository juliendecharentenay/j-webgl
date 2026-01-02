use super::{Error, Result, traits, };
use wasm_bindgen::JsCast;

/// Renderer for WebGL 2.0 rendering operations
pub struct Renderer {
    context: web_sys::WebGl2RenderingContext,
    canvas: web_sys::HtmlCanvasElement,
    renderables: Vec<(String, Box<dyn traits::Renderable>)>,
}

impl Renderer {
    pub fn with_renderable(&mut self, id: String, renderable: Option<Box<dyn traits::Renderable>>) -> Result<()> {
      if let Some(r) = renderable {
        if let Some(i) = self.renderables.iter().position(|(s, _)| s.eq(&id)) {
          self.renderables[i] = (id, r);
        } else {
          self.renderables.push((id, r));
        }
      } else {
        self.renderables.retain(|(s, _)| s.ne(&id));
      }

      Ok(())
    }

    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<Self> {
        let context = canvas
            .get_context("webgl2")
            .map_err(|e| { Error::UnsupportedOperation(format!("Failed to get WebGL 2.0 context: {:?}", e)) })?
            .ok_or_else(|| Error::UnsupportedOperation("WebGL 2.0 context is null".to_string()))?
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .map_err(|_| Error::UnsupportedOperation("Failed to cast to WebGL 2.0 context".to_string()))?;

        let renderer = Renderer {
            context,
            canvas,
            renderables: Vec::new(),
        };

        // Resize the canvas to match its CSS size, accounting for device pixel ratio
        renderer.resize()?;

        Ok(renderer)
    }

    pub fn render<C: traits::Camera>(&self, camera: &C) -> Result<()> {
      self.context.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);
      /*
      use traits::Camera;
      let camera = structs::camera::Basic::new(
        self.canvas.client_width() as f32, self.canvas.client_height() as f32,
      )?;
      */
      for (_, r) in self.renderables.iter() {
        r.render(&self.context, camera.as_f32_array()?)?;
      }
      Ok(())
    }

    /// Resizes the canvas to match its CSS size, accounting for device pixel ratio
    pub fn resize(&self) -> Result<()> {
        let window = web_sys::window()
            .ok_or_else(|| Error::UnsupportedOperation("Window object not available".to_string()))?;
        
        let dpr = window.device_pixel_ratio();
        let rect = self.canvas.get_bounding_client_rect();
        
        self.canvas.set_width((rect.width() * dpr) as u32);
        self.canvas.set_height((rect.height() * dpr) as u32);
        
        self.context.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);
        
        Ok(())
    }

    pub fn resize_and_render<C: traits::Camera>(&self, camera: &C) -> Result<()> {
        self.resize()?;
        self.render(camera)?;
        Ok(())
    }
}

