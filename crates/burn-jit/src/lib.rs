#![warn(missing_docs)]

//! Burn JIT Backend

#[macro_use]
extern crate derive_new;
extern crate alloc;
extern crate core;

mod ops;

/// Compute related module.
pub mod compute;
/// Kernel module
pub mod kernel;
/// Tensor module.
pub mod tensor;

pub(crate) mod codegen;
pub(crate) mod tune;

mod element;
pub use codegen::compiler::Compiler;
pub use codegen::dialect::gpu;

pub use element::{FloatElement, IntElement, JitElement};

mod backend;
pub use backend::*;
mod runtime;
pub use runtime::*;

#[cfg(any(feature = "fusion", test))]
mod fusion;

#[cfg(feature = "export_tests")]
pub mod tests;
