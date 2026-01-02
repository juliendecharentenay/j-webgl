use super::{Result, Error};

pub fn compile_program<V, F>(context: &web_sys::WebGl2RenderingContext,
  vertex_shader_source: V,
  fragment_shader_source: F,
) -> Result<web_sys::WebGlProgram> 
where V: std::convert::AsRef<str>,
      F: std::convert::AsRef<str>,
{
  let vertex_shader = compile_shader(context,
    web_sys::WebGl2RenderingContext::VERTEX_SHADER,
    vertex_shader_source
  )?;
  let fragment_shader = compile_shader(context,
    web_sys::WebGl2RenderingContext::FRAGMENT_SHADER,
    fragment_shader_source
  )?;

  let program = context
    .create_program()
    .ok_or_else(|| Error::UnsupportedOperation("Failed to create shader program".to_string()))?;

  context.attach_shader(&program, &vertex_shader);
  context.attach_shader(&program, &fragment_shader);
  context.link_program(&program);

  if !context.get_program_parameter(&program, web_sys::WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
  {
    let error = context
      .get_program_info_log(&program)
      .unwrap_or_else(|| "Unknown error".to_string());
    return Err(Error::UnsupportedOperation(format!("Shader program linking failed: {}", error)));
  }
  Ok(program)
}

pub fn compile_shader<T: std::convert::AsRef<str>>(context: &web_sys::WebGl2RenderingContext,
        shader_type: u32,
        source: T,
) -> Result<web_sys::WebGlShader> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| Error::UnsupportedOperation("Failed to create shader".to_string()))?;

  context.shader_source(&shader, source.as_ref());
  context.compile_shader(&shader);

  if !context
      .get_shader_parameter(&shader, web_sys::WebGl2RenderingContext::COMPILE_STATUS)
      .as_bool()
      .unwrap_or(false)
  {
    let error = context
      .get_shader_info_log(&shader)
      .unwrap_or_else(|| "Unknown error".to_string());
    return Err(Error::UnsupportedOperation(format!("Shader compilation failed: {}", error)));
  }

  Ok(shader)
}

