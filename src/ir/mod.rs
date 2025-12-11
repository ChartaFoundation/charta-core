pub mod validation;
pub mod schema;

pub use validation::validate_ir;
pub use schema::{load_schema, IR, Module, SignalDecl, CoilDecl, RungDecl, GuardExpr, Action};
