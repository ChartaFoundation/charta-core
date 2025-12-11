pub mod ir;
pub mod error;

pub use ir::{validate_ir, load_schema, IR};
pub use error::{ValidationError, Result};
