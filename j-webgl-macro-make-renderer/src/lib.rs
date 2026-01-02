use proc_macro::TokenStream;

mod make_renderer;

#[proc_macro]
pub fn make_renderer(input: TokenStream) -> TokenStream {
  make_renderer::expand_macro(input.into())
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}
