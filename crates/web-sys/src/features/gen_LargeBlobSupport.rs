#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `LargeBlobSupport` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `LargeBlobSupport`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeBlobSupport {
    Required = "required",
    Preferred = "preferred",
}
