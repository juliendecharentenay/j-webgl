j_webgl_macro_make_renderer::make_renderer!({
  name: MyRenderer,
  renderables: [ 
    j_webgl::structs::renderable::Initializer, 
    j_webgl::structs::renderable::Cube, 
    j_webgl::structs::renderable::CubeWithNormals, 
  ],
  camera: j_webgl::structs::camera::Basic,
});

