use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{braced, Expr, ExprArray, Ident, Result as SynResult, Token, Type, TypePath, spanned::Spanned};

struct MacroInput {
    name: Ident,
    renderables: Vec<Type>,
    camera: Type,
}

impl Parse for MacroInput {
  fn parse(input: ParseStream) -> SynResult<Self> {
      let content;
      braced!(content in input);

      let mut name: Option<Ident> = None;
      let mut renderables: Option<Vec<Type>> = None;
      let mut camera: Option<Type> = None;

      // Loop through fields in any order
      while !content.is_empty() {
          let field_ident: Ident = content.parse()?;
          let field_name = field_ident.to_string();
          
          content.parse::<Token![:]>()?;

          match field_name.as_str() {
              "name" => {
                  if name.is_some() {
                      return Err(syn::Error::new(field_ident.span(), "duplicate `name` field"));
                  }
                  let parsed_name: Ident = content.parse()?;
                  name = Some(parsed_name);
              }
              "renderables" => {
                  if renderables.is_some() {
                      return Err(syn::Error::new(field_ident.span(), "duplicate `renderables` field"));
                  }
                  let array: ExprArray = content.parse()?;
                  
                  // Extract types from the array
                  let mut parsed_renderables = Vec::new();
                  for elem in array.elems {
                      if let Expr::Path(expr_path) = elem {
                          let type_path = Type::Path(TypePath {
                            qself: None,
                            path: expr_path.path,
                          });
                          parsed_renderables.push(type_path);
                      } else {
                          return Err(syn::Error::new(elem.span(), "expected a type"));
                      }
                  }
                  renderables = Some(parsed_renderables);
              }
              "camera" => {
                  if camera.is_some() {
                      return Err(syn::Error::new(field_ident.span(), "duplicate `camera` field"));
                  }
                  let parsed_camera: Type = content.parse()?;
                  camera = Some(parsed_camera);
              }
              _ => {
                  return Err(syn::Error::new(field_ident.span(), 
                      format!("unknown field `{}`, expected `name` or `renderables` or `camera`", field_name)));
              }
          }

          // Parse optional trailing comma
          if !content.is_empty() {
              content.parse::<Option<Token![,]>>()?;
          }
      }

      // Ensure both required fields are present
      let name = name.ok_or_else(|| {
          syn::Error::new(content.span(), "missing required field `name`")
      })?;
      
      let renderables = renderables.ok_or_else(|| {
          syn::Error::new(content.span(), "missing required field `renderables`")
      })?;

      let camera = camera.ok_or_else(|| {
          syn::Error::new(content.span(), "missing required field `camera`")
      })?;

      Ok(MacroInput { name, renderables, camera })
  }
}


fn pascal_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    if let Some(first) = chars.next() {
        result.push(first.to_lowercase().next().unwrap_or(first));
    }
    
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            result.push('_');
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            result.push(ch);
        }
    }
    
    result
}

fn get_last_segment_name(ty: &Type) -> Option<String> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return Some(pascal_to_snake_case(&segment.ident.to_string()));
        }
    }
    None
}

pub fn expand_macro(tokens: TokenStream) -> SynResult<TokenStream> {
    let input: MacroInput = syn::parse2(tokens)?;

    let struct_name = &input.name;
    let renderable_types = &input.renderables;
    let camera_type = &input.camera;
    
    // Generate the struct
    let struct_def = quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub struct #struct_name {
            inner: webgl_rs::Renderer,
        }
    };

    // Generate the impl block with methods
    let mut methods = Vec::new();

    // new() method
    let new_method = quote! {
        #[wasm_bindgen(constructor)]
        pub fn new(canvas: web_sys::HtmlCanvasElement) -> std::result::Result<#struct_name, wasm_bindgen::JsValue> {
            Ok(#struct_name {
                inner: webgl_rs::Renderer::new(canvas)?,
            })
        }
    };
    methods.push(new_method);

    // with_* methods for each renderable type
    for renderable_type in renderable_types {
        // Extract the last segment name and convert to lowercase
        let method_suffix = get_last_segment_name(renderable_type)
            .unwrap_or_else(|| "initializer".to_string());
        let method_name = format_ident!("with_{}", method_suffix);
        
        let method = quote! {
            pub fn #method_name(mut self, id: String, renderable: #renderable_type) -> std::result::Result<Self, wasm_bindgen::JsValue> {
                self.inner.with_renderable(id, Some(Box::new(renderable)))?;
                Ok(self)
            }
        };
        methods.push(method);
    }

    // without method to remove a renderables
    let without_method = quote! {
        pub fn without(mut self, id: String) -> std::result::Result<Self, wasm_bindgen::JsValue> {
            self.inner.with_renderable(id, None)?;
            Ok(self)
        }
    };
    methods.push(without_method);

    // resize method to handle canvas resize
    let resize_method = quote! {
        pub fn resize(&self) -> std::result::Result<(), wasm_bindgen::JsValue> {
            Ok(self.inner.resize()?)
        }
    };
    methods.push(resize_method);

    // render method to render the scene
    let render_method = quote! {
        pub fn render(&self, camera: &#camera_type) -> std::result::Result<(), wasm_bindgen::JsValue> {
            Ok(self.inner.render(camera)?)
        }
    };
    methods.push(render_method);

    let impl_block = quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl #struct_name {
            #(#methods)*
        }
    };

    Ok(quote! {
        #struct_def
        #impl_block
    })
}
