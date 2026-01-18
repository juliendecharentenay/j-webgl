use super::*;


#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
pub struct CubeWithNormals {
  inner: std::cell::RefCell<Option<programs::TrianglesShaded>>,
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
    camera_mvp: &[f32],
  ) -> Result<()>
  {
    if self.inner.borrow().is_none() {
      let mut program = programs::TrianglesShaded::new(context)?;
      program.with_color(
        0.2, // R
        1.0, // G
        0.2, // B
        1.0, // Alpha
      )?;
      program.with_reverse_light_direction([0.5, 0.7, 1.0])?;
      program.with_position(12, get_positions().as_slice())?;
      program.with_normals(get_normals().as_slice())?;
      *self.inner.borrow_mut() = Some(program);
    }

    let mut binding = self.inner.borrow_mut();
    let inner = binding.as_mut().unwrap();
    inner.with_mvp(camera_mvp)?;
    inner.draw()?;

    Ok(())
  }
}

fn get_positions() -> Vec<f32> {
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
  let array: Vec<f32> = indices.iter()
  .fold(Vec::new(), |mut a, i| {
          a.push(vertices[*i*3]);
          a.push(vertices[*i*3+1]);
          a.push(vertices[*i*3+2]);
          a
  });
  array
}

fn get_normals() -> Vec<f32> {
  let mut normals = Vec::<f32>::new();
  for _ in 0..6 { normals.append(&mut vec![ 0.0, 0.0, 1.0, ]); }  // Front face
  for _ in 0..6 { normals.append(&mut vec![ 0.0, 0.0, -1.0, ]); } // Back face
  for _ in 0..6 { normals.append(&mut vec![ 0.0, 1.0,  0.0, ]); } // Top face
  for _ in 0..6 { normals.append(&mut vec![ 0.0, -1.0, 0.0, ]); } // Bottom face
  for _ in 0..6 { normals.append(&mut vec![ 1.0, 0.0,  0.0, ]); } // Right face
  for _ in 0..6 { normals.append(&mut vec![-1.0, 0.0,  0.0, ]); } // Left face
  normals
}

