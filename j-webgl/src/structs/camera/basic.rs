use super::*;

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Basic {
  matrix: algebra::Matrix4,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Basic {
  #[wasm_bindgen(constructor)]
  pub fn new(width: f32, height: f32) -> Result<Self> {
    let projection = algebra::Matrix4::new_perspective(
          width / height,
          45.0 * std::f32::consts::PI / 180f32, // fov
          0.1f32,
          200f32,
    );

    let eye = algebra::Point3::new( EYE_X, EYE_Y, EYE_Z);
    let target = algebra::Point3::new(0.0, 0.0, 0.0);
    let up = algebra::Vector3::new(0.0, 1.0, 0.0);
    let view = algebra::Matrix4::look_at_rh(&eye, &target, &up);

    let matrix = &projection * &view;
    Ok(Basic { matrix })
  }
}

impl traits::Camera for Basic {
  fn as_f32_array(&self) -> Result<&[f32]> {
    Ok(self.matrix.as_slice())
  }
}
