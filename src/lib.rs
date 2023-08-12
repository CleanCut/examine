//! Please see [the project README](https://github.com/CleanCut/examine) for a general description
//! of the project and how to use it.
//!
//! If this were a "real" project, most of that documentation would be on this page instead.
//!
//! The one item that has a bunch of good documentation and doc-tests is the [`Examine`](crate::service::Examine) struct.

/// Module containing generated items
pub mod gen;

/// Module containing manually-written 'examine' service code used by the server and tests. This is
/// where the *actual implementation* of all the service logic is!!!
pub mod service;

// Expose the generated items at the root library level for convenience
pub use gen::examine::v1::*;
