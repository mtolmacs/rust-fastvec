#![warn(missing_docs)]

mod stack;

/// Contains the error type and all error variations this crate can return
pub mod error;

/// This is the core functinoality, containing the FastVec structure and
/// implementation
pub mod fastvec;
pub mod deref;
pub mod index;

pub use crate::fastvec::FastVec;

/// This is a test
#[cfg(test)]
mod tests;
