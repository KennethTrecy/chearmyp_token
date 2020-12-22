use alloc::collections::VecDeque;
use crate::lex::Token;

/// Contains the tokens that have been lexed.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct TokenStream<'a>(pub VecDeque<Token<'a>>);

impl<'a> TokenStream<'a> {
	pub fn new() -> Self {
		TokenStream(VecDeque::new())
	}
}

#[cfg(test)]
use core::cmp::PartialEq;

#[cfg(test)]
impl<'a> PartialEq<VecDeque<Token<'a>>> for TokenStream<'a> {
	fn eq(&self, other: &VecDeque<Token<'a>>) -> bool {
		PartialEq::eq(&self.0, other)
	}
}

#[cfg(test)]
mod t {
	use super::VecDeque;
	use super::TokenStream;

	#[test]
	fn can_compare_to_itself() {
		let stream = TokenStream(VecDeque::new());
		let other_stream = TokenStream(VecDeque::new());
		assert_eq!(stream, other_stream);
	}
}
