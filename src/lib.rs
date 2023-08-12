// Module to contain generated items
pub mod gen;
// Module to contain manually-written 'examine' service code used by the server and tests
pub mod service;

// Expose the generated items at the root library level for convenience
pub use gen::examine::v1::*;
