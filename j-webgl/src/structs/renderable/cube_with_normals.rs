use super::*;

struct Inner {
  program: web_sys::WebGlProgram,
  vao: web_sys::WebGlVertexArrayObject,
  matrix_location: web_sys::WebGlUniformLocation,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
pub struct CubeWithNormals {
  inner: std::cell::RefCell<Option<Inner>>,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl CubeWithNormals {
  pub fn new() -> Result<CubeWithNormals> {
    Ok(CubeWithNormals { inner: std::cell::RefCell::new(None), })
  }
}

impl traits::Renderable for CubeWithNormals {
  fn render(
    &self,
    context: &web_sys::WebGl2RenderingContext,
    camera: &[f32],
  ) -> Result<()>
  {
    if self.inner.borrow().is_none() {
      let program = utils::compile_program(
        context,
        shaders::vertex::matrix_position_normal()?,
        shaders::fragment::color_light()?,
      )?;

      let position_attribute_location = context.get_attrib_location(&program, "a_position");
      let normal_attribute_location = context.get_attrib_location(&program, "a_normal");
      let matrix_location = context.get_uniform_location(&program, "u_matrix");
      let color_location = context.get_uniform_location(&program, "u_color");
      let reverse_light_direction_location = context.get_uniform_location(&program, "u_reverseLightDirection");

      let vertices: [f32; 24] = [
            // Front face
            -0.5, -0.5,  0.5,  // 0: bottom-left-front
             0.5, -0.5,  0.5,  // 1: bottom-right-front
             0.5,  0.5,  0.5,  // 2: top-right-front
            -0.5,  0.5,  0.5,  // 3: top-left-front
            // Back face
            -0.5, -0.5, -0.5,  // 4: bottom-left-back
             0.5, -0.5, -0.5,  // 5: bottom-right-back
             0.5,  0.5, -0.5,  // 6: top-right-back
            -0.5,  0.5, -0.5,  // 7: top-left-back
      ];

      // Cube indices: 12 triangles (2 per face) = 36 indices
      let indices: [usize; 36] = [
            // Front face
            0, 1, 2,  2, 3, 0,
            // Back face
            4, 6, 5,  6, 4, 7,
            // Top face
            3, 2, 6,  6, 7, 3,
            // Bottom face
            0, 4, 5,  5, 1, 0,
            // Right face
            1, 5, 6,  6, 2, 1,
            // Left face
            0, 3, 7,  7, 4, 0,
      ];
      let mut normals = Vec::<f32>::new();
      for _ in 0..6 { normals.append(&mut vec![ 0.0, 0.0, 1.0, ]); }  // Front face
      for _ in 0..6 { normals.append(&mut vec![ 0.0, 0.0, -1.0, ]); } // Back face
      for _ in 0..6 { normals.append(&mut vec![ 0.0, 1.0,  0.0, ]); } // Top face
      for _ in 0..6 { normals.append(&mut vec![ 0.0, -1.0, 0.0, ]); } // Bottom face
      for _ in 0..6 { normals.append(&mut vec![ 1.0, 0.0,  0.0, ]); } // Right face
      for _ in 0..6 { normals.append(&mut vec![-1.0, 0.0,  0.0, ]); } // Left face

      let array: Vec<f32> = indices.iter()
      .fold(Vec::new(), |mut a, i| {
          a.push(vertices[*i*3]);
          a.push(vertices[*i*3+1]);
          a.push(vertices[*i*3+2]);
          a
      });

      // Position
      let position_buffer = context.create_buffer().unwrap();
      let vao = context.create_vertex_array().unwrap();
      context.bind_vertex_array(Some(&vao));
      context.enable_vertex_attrib_array(position_attribute_location as u32);
      context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
      unsafe {
          let vertices_array = js_sys::Float32Array::view(array.as_slice());
          context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
          );
      }
      context.vertex_attrib_pointer_with_i32(position_attribute_location as u32,
          3, // size
          web_sys::WebGl2RenderingContext::FLOAT,
          false, // don't normalize
          0, // stride
          0, // offset
      );


      // Normals
      let normal_buffer = context.create_buffer().unwrap();
      context.enable_vertex_attrib_array(normal_attribute_location as u32);
      context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&normal_buffer));
      unsafe {
          let array = js_sys::Float32Array::view(normals.as_slice());
          context.buffer_data_with_array_buffer_view(
                web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
                &array,
                web_sys::WebGl2RenderingContext::STATIC_DRAW,
          );
      }
      context.vertex_attrib_pointer_with_i32(normal_attribute_location as u32,
          3, // size
          web_sys::WebGl2RenderingContext::FLOAT,
          false, // don't normalize
          0, // stride
          0, // offset
      );

      // Color
      context.use_program(Some(&program));
      context.uniform4f(
        color_location.as_ref(),
        0.2, // R
        1.0, // G
        0.2, // B
        1.0, // Alpha
      );

      // Light
      context.uniform3f(
        reverse_light_direction_location.as_ref(),
        0.5, 0.7, 1.0,
      );

      *self.inner.borrow_mut() = Some(
        Inner {
          program,
          vao,
          matrix_location: matrix_location.unwrap(),
        }
      );
    }

    let binding = self.inner.borrow();
    let inner = binding.as_ref().unwrap();

    context.use_program(Some(&inner.program));
    context.bind_vertex_array(Some(&inner.vao));
    context.uniform_matrix4fv_with_f32_array(
      Some(&inner.matrix_location),
      false,
      camera,
    );
    context.draw_arrays(
          web_sys::WebGl2RenderingContext::TRIANGLES,
          0, // offset
          12 * 3, // count
    );

    Ok(())
  }
}
