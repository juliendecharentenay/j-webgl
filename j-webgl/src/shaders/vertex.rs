use super::*;

pub fn position_matrix() -> Result<String> {
  Ok(format!(
         r#"#version 300 es
            in vec4 a_position;
            uniform mat4 u_matrix;
            
            void main() {{
                gl_Position = u_matrix * a_position;
            }}
          "#
  ))
}

pub fn matrix_position_normal() -> Result<String> {
  Ok(
    r#"#version 300 es

       // an attribute is an input (in) to a vertex shader.
       // It will receive data from a buffer
       in vec4 a_position;
       in vec3 a_normal;

       // A matrix to transform the positions by
       uniform mat4 u_matrix;

       // varying to pass the normal to the fragment shader
       out vec3 v_normal;

       // all shaders have a main function
       void main() {
         // Multiply the position by the matrix.
         gl_Position = u_matrix * a_position;
       
         // Pass the normal to the fragment shader
         v_normal = a_normal;
       }
     "#
     .to_string()
  )
}
