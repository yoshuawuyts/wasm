//! WebAssembly component types.
//!
//! This module groups the compiled-component data models:
//! `WasmComponent` and `ComponentTarget`.

mod models;

#[allow(unused_imports, unreachable_pub)]
pub use models::{ComponentTarget, WasmComponent};
