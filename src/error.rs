use std::fmt::Display;

/// This is the Error type for FastVec which can signal the source of the issue
/// if something goes wrong.
/// 
// TODO(mtolmacs): Allow packaging the source errors
#[derive(Debug)]
pub enum Error {
    /// OS memory allocation was unsuccessful
    AllocError,

    /// The memory layout requested is incorrect (should not show up at all,
    /// it indicates an internal error)
    LayoutError,

    /// The buffer cannot store more items (too numerous)
    CapacityOverflow,
}

// NOTE: Required for implementing std::error:Error
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AllocError => write!(f, "Allocation error"),
            Error::LayoutError => write!(f, "Memory layout error"),
            Error::CapacityOverflow => write!(f, "Allocation too large"),
        }
    }
}

impl std::error::Error for Error {}