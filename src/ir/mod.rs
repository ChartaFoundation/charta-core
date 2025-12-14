pub mod validation;
pub mod schema;
pub mod evidence;

pub use validation::validate_ir;
pub use schema::{load_schema, IR, Module, SignalDecl, CoilDecl, RungDecl, GuardExpr, Action};
pub use evidence::{Evidence, EvidenceSource, EvidenceType, EvidenceBool, EvidenceNumeric, EvidenceText};
