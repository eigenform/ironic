//! Utilities for implementing different kinds of backends.

/// Common interface implemented by different backends.
pub trait Backend {
    /// The main loop for this particular backend.
    fn run(&mut self);
}
