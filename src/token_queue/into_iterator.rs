use alloc::collections::vec_deque::IntoIter;
use core::iter::IntoIterator;
use crate::Token;
use super::TokenQueue;

impl<'a> IntoIterator for TokenQueue<'a> {
	type Item = Token<'a>;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

#[cfg(test)]
mod t {
	use super::{Token, TokenQueue};

	#[test]
	fn can_convert_into_iterator() {
		let token = Token::Simplex(b"abcdefg");
		let expected_token = Token::Simplex(b"abcdefg");
		let mut token_queue = TokenQueue::new();
		token_queue.push(token);

		let mut iterator = token_queue.into_iter();
		assert_eq!(iterator.next(), Some(expected_token));
		assert_eq!(iterator.next(), None);
	}
}
