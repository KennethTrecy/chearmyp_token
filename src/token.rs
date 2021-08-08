use alloc::vec::Vec;

/// Contains the valid tokens used for lexing and parsing.
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
	ScopeLevel(usize),
	LineComment(&'a [u8]),
	BlockComment(Vec<&'a [u8]>),
	Simplex(&'a [u8]),
	Complex(&'a [u8]),
	Attacher(&'a [u8], &'a [u8]),
	LineOthertongue(&'a [u8]),
	BlockOthertongue(Vec<&'a [u8]>)
}
