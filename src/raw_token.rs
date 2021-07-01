use alloc::vec::Vec;

/// Contains the raw tokens used for lexing only.
#[derive(Debug, PartialEq)]
pub enum RawToken<'a> {
	Empty,
	Invalid,
	ScopeLevel(usize),
	Block(Vec<&'a [u8]>),
	LineComment(&'a [u8]),
	BlockComment(Vec<&'a [u8]>),
	Simplex(&'a [u8]),
	Complex(&'a [u8]),
	Attacher(&'a [u8], &'a [u8]),
	LineOthertongue(&'a [u8]),
	BlockOthertongue(Vec<&'a [u8]>)
}

/// Contains the extracted raw token and its last index occupied in the source.
/// This raw token is used as return value for most lexers.
pub type RawTokenInfo<'a> = (RawToken<'a>, usize);
