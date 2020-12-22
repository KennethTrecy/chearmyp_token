use alloc::collections::VecDeque;
use crate::lex::Token;

/// Contains the tokens that have been lexed.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct TokenQueue<'a>(pub VecDeque<Token<'a>>);

impl<'a> TokenQueue<'a> {
	pub fn new() -> Self {
		TokenQueue(VecDeque::new())
	}
}

#[cfg(test)]
use core::cmp::PartialEq;

#[cfg(test)]
impl<'a> PartialEq<VecDeque<Token<'a>>> for TokenQueue<'a> {
	fn eq(&self, other: &VecDeque<Token<'a>>) -> bool {
		PartialEq::eq(&self.0, other)
	}
}

#[cfg(test)]
mod t {
	use super::VecDeque;
	use super::TokenQueue;

	#[test]
	fn can_compare_to_itself() {
		let queue = TokenQueue(VecDeque::new());
		let other_queue = TokenQueue(VecDeque::new());
		assert_eq!(queue, other_queue);
	}
}
