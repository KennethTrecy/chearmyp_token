use alloc::vec::Vec;

/// Contains the tokens used for lexing.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Token<'a> {
	Empty,
	Invalid,
	LineComment(&'a [u8]),
	BlockComment(Vec<&'a [u8]>),
	Simplex(&'a [u8])
}

/// Contains the extracted token and its last index occupied in the source.
/// This token is used as return value for lexers.
pub type TokenInfo<'a> = (Token<'a>, usize);
