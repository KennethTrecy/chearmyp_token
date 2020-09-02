#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Token<'a> {
	Empty,
	LineComment(&'a [u8])
}

pub struct TokenInfo<'a>(pub Token<'a>, pub usize);
