/// Contains the tokens used for lexing.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Token<'a> {
	Empty,
	LineComment(&'a [u8])
}

/// Contains the extracted token and its last index occupied in the source.
/// This token is used as return value for lexers.
pub struct TokenInfo<'a>(pub Token<'a>, pub usize);
