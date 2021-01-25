//! Message passing between backend elements.

use std::sync::mpsc::{Sender, Receiver};

/// Thread control messages.
pub enum ThreadMsg {
    HaltThread,
}

