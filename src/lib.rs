#![no_std]
extern crate alloc;

/// Contains the data structures and type aliases used and/or returned by some lexers. They can be
/// used by both lexers and parsers.
mod token;

pub use token::Token;
