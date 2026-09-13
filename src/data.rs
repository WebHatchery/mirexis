//! Embedded content schemas and registry validation.
pub mod schema;
pub use schema::*;

pub mod loader;
pub mod operation;
pub mod techniques;
pub mod validation;
pub use operation::OperationModifier;
pub use techniques::{TechniqueDef, TechniqueTarget};
pub use validation::ensure_unique;
