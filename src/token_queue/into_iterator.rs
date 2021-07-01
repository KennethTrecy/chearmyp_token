use alloc::collections::vec_deque::IntoIter;
use core::iter::IntoIterator;
use crate::RawToken;
use super::TokenQueue;

impl<'a> IntoIterator for TokenQueue<'a> {
	type Item = RawToken<'a>;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

#[cfg(test)]
mod t {
	use super::{RawToken, TokenQueue};

	#[test]
	fn can_convert_into_iterator() {
		let raw_token = RawToken::Empty;
		let expected_token = RawToken::Empty;
		let mut token_queue = TokenQueue::new();
		token_queue.push(raw_token);

		let mut iterator = token_queue.into_iter();
		assert_eq!(iterator.next(), Some(expected_token));
		assert_eq!(iterator.next(), None);
	}
}
