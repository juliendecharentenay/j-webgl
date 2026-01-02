/// Generic error type for WebGL operations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("FromJsValue: {0}")]
    FromJsValue(String),
    /*
    #[error(transparent)]
    SerdeWasmBindgenError(#[from] serde_wasm_bindgen::Error),
    */

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Error::FromJsValue(value.as_string().unwrap_or_default())
    }
}

impl From<Error> for wasm_bindgen::JsValue {
    fn from(error: Error) -> Self {
        wasm_bindgen::JsValue::from_str(&error.to_string())
    }
}

/// Result type alias using Error
pub type Result<T> = std::result::Result<T, Error>;
