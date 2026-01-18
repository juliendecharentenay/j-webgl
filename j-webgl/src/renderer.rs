use super::{Error, Result, traits, };
use wasm_bindgen::JsCast;

/// Renderer for WebGL 2.0 rendering operations
///
/// The `Renderer` manages a WebGL 2.0 context and a collection of renderable objects.
/// It handles canvas resizing, viewport management, and orchestrates the rendering
/// of all registered renderable objects.
///
/// # Example
///
/// ```rust
/// use j_webgl::{Renderer, Result, Renderable, Camera};
/// use web_sys::HtmlCanvasElement;
/// use wasm_bindgen::JsCast;
///
/// # fn example() -> Result<()> {
/// // Assuming you have a canvas element
/// # let canvas: HtmlCanvasElement = unimplemented!();
/// let mut renderer = Renderer::new(canvas)?;
///
/// // Add renderable objects
/// // renderer.with_renderable("obj1".to_string(), Some(my_renderable))?;
///
/// // Render with a camera
/// // let camera = MyCamera::new();
/// // renderer.render(&camera)?;
/// # Ok(())
/// # }
/// ```
///
pub struct Renderer {
    context: web_sys::WebGl2RenderingContext,
    canvas: web_sys::HtmlCanvasElement,
    renderables: Vec<(String, Box<dyn traits::Renderable>)>,
}

impl Renderer {
      /// Add, update, or remove a renderable object
    ///
    /// This method manages renderable objects in the renderer's collection:
    /// - If `renderable` is `Some`, the object is added or updated (if an object
    ///   with the same `id` already exists)
    /// - If `renderable` is `None`, the object with the given `id` is removed
    ///
    /// # Parameters
    ///
    /// - `id`: A unique identifier for the renderable object
    /// - `renderable`: An optional renderable object to add/update, or `None` to remove
    /// # Example
    ///
    /// ```rust
    /// # use j_webgl::{Renderer, Result, Renderable};
    /// # fn example(mut renderer: Renderer, my_obj: impl Renderable + 'static) -> Result<()> {
    /// // Add a renderable
    /// renderer.with_renderable("cube".to_string(), Some(my_obj))?;
    ///
    /// // Remove it later
    /// renderer.with_renderable("cube".to_string(), None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_renderable<R: traits::Renderable + 'static>(&mut self, id: String, renderable: Option<R>) -> Result<()> {
      if let Some(r) = renderable {
        if let Some(i) = self.renderables.iter().position(|(s, _)| s.eq(&id)) {
          self.renderables[i] = (id, Box::new(r));
        } else {
          self.renderables.push((id, Box::new(r)));
        }
      } else {
        self.renderables.retain(|(s, _)| s.ne(&id));
      }

      Ok(())
    }

    /// Create a new renderer from a canvas element
    ///
    /// This initializes a WebGL 2.0 context from the provided canvas and sets up
    /// the renderer. The canvas is automatically resized to match its CSS size,
    /// accounting for the device pixel ratio.
    ///
    /// # Parameters
    ///
    /// - `canvas`: The HTML canvas element to use for rendering
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - WebGL 2.0 is not supported
    /// - The context cannot be created
    /// - Canvas resizing fails
    ///
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

    /// Render all registered renderable objects
    ///
    /// This method sets up the viewport and renders all registered renderable objects
    /// using the provided camera's MVP matrix.
    ///
    /// # Parameters
    ///
    /// - `camera`: A camera implementation that provides the MVP matrix
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The camera cannot provide its MVP matrix
    /// - Any renderable object fails to render

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

    /// Resize the canvas to match its CSS size, accounting for device pixel ratio
    ///
    /// This method should be called when the canvas size changes (e.g., window resize).
    /// It updates both the canvas's internal size and the WebGL viewport.
    ///
    /// # Errors
    ///
    /// Returns an error if the window object is not available.
    ///
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

    /// Resize the canvas and render in one operation
    ///
    /// This is a convenience method that combines [`resize`](Self::resize) and
    /// [`render`](Self::render) into a single call. Useful for handling window
    /// resize events.
    ///
    /// # Parameters
    ///
    /// - `camera`: A camera implementation that provides the MVP matrix
    ///
    /// # Errors
    ///
    /// Returns an error if resizing or rendering fails.
    pub fn resize_and_render<C: traits::Camera>(&self, camera: &C) -> Result<()> {
        self.resize()?;
        self.render(camera)?;
        Ok(())
    }
}

