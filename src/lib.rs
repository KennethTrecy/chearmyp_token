#![no_std]
extern crate alloc;

/// Contains the data structures and type aliases used and/or returned by different lexers.
pub mod token;

/// Contains different characters needed to be recognized by the different lexers.
pub mod special_characters;

/// Contains `find_line_ending()`.
mod find_line_ending;

/// Contains `line_comment()` lexer.
pub mod line_comment;

/// Contains `block_comment()` lexer.
pub mod block_comment;

/// Contains `simplex()` lexer.
pub mod simplex;

/// Contains `complex()` lexer.
pub mod complex;

/// Contains `attacher()` lexer.
pub mod attacher;

/// Contains `line_othertongue()` lexer.
pub mod line_othertongue;

/// Contains `block()` lexer.
pub mod block;

/// Contains types of delimeter that lexers search for.
mod delimeter;

pub use find_line_ending::find_line_ending;
