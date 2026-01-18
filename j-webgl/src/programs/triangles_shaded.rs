use super::*;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject, WebGlBuffer, };

pub struct TrianglesShaded {
  context: WebGl2RenderingContext,
  program: WebGlProgram,
  vertex_array_object: Option<WebGlVertexArrayObject>,
  position_buffer: Option<WebGlBuffer>,
  n_triangles: Option<usize>,
  normal_buffer: Option<WebGlBuffer>,
}

impl TrianglesShaded {
  pub fn new(context: &WebGl2RenderingContext) -> Result<Self> {
    let context = context.clone();
    let program = utils::compile_program(
      &context,
      shaders::vertex::matrix_position_normal()?,
      shaders::fragment::color_light()?,
    )?;
    Ok(TrianglesShaded { 
      context,
      program, 
      vertex_array_object: None, 
      position_buffer: None,
      n_triangles: None,
      normal_buffer: None,
    })
  }

  pub fn with_mvp(&mut self, mvp: &[f32]) -> Result<()> {
    let context = &self.context;
    context.use_program(Some(&self.program));
    let location = context.get_uniform_location(&self.program, "u_matrix");
    context.uniform_matrix4fv_with_f32_array(location.as_ref(), false, mvp);
    Ok(())
  }

  pub fn with_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Result<()> {
    let context = &self.context;
    context.use_program(Some(&self.program));
    let location = context.get_uniform_location(&self.program, "u_color");
    context.uniform4f(location.as_ref(), red, green, blue, alpha);
    Ok(())
  }

  pub fn with_reverse_light_direction(&mut self, vector: [f32; 3]) -> Result<()> {
    let context = &self.context;
    context.use_program(Some(&self.program));
    let location = context.get_uniform_location(&self.program, "u_reverseLightDirection");
    context.uniform3fv_with_f32_array(location.as_ref(), &vector);
    Ok(())
  }

  pub fn with_normals(&mut self, normals: &[f32]) -> Result<()> {
    let context = &self.context;
    if self.normal_buffer.is_none() {
      self.normal_buffer = context.create_buffer();
    }
    context.use_program(Some(&self.program));
    let location = context.get_attrib_location(&self.program, "a_normal");
    context.enable_vertex_attrib_array(location as u32);
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.normal_buffer.as_ref());
    unsafe {
      let array = js_sys::Float32Array::view(normals);
      context.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &array,
        WebGl2RenderingContext::STATIC_DRAW
      );
    }
    context.vertex_attrib_pointer_with_i32(location as u32,
      3, // size
      WebGl2RenderingContext::FLOAT,
      false, // don't normalize
      0, // stride
      0, // offset
    );
    Ok(())
  }

  pub fn with_position(&mut self, n_triangles: usize, values: &[f32]) -> Result<()> {
    let context = &self.context;
    self.n_triangles = Some(n_triangles);
    if self.position_buffer.is_none() {
      self.position_buffer = context.create_buffer();
    }
    if self.vertex_array_object.is_none() {
      self.vertex_array_object = context.create_vertex_array();
    }

    context.use_program(Some(&self.program));
    let location = context.get_attrib_location(&self.program, "a_position");
    context.bind_vertex_array(self.vertex_array_object.as_ref());
    context.enable_vertex_attrib_array(location as u32);
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, self.position_buffer.as_ref());
    unsafe {
          let vertices_array = js_sys::Float32Array::view(values);
          context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
          );
    }
    context.vertex_attrib_pointer_with_i32(location as u32,
          3, // size
          web_sys::WebGl2RenderingContext::FLOAT,
          false, // don't normalize
          0, // stride
          0, // offset
    );

    Ok(())
  }

  pub fn draw(&self) -> Result<()> {
    let context = &self.context;
    context.use_program(Some(&self.program));
    context.draw_arrays(
      web_sys::WebGl2RenderingContext::TRIANGLES,
      0, // offset
      (self.n_triangles.unwrap_or_default() * 3).try_into()?, // count
    );

    Ok(())
  }
}
