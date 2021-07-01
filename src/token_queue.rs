use alloc::collections::VecDeque;
use crate::RawToken;

/// Contains the tokens that have been lexed.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct TokenQueue<'a>(pub VecDeque<RawToken<'a>>);

impl<'a> TokenQueue<'a> {
	/// Creates the raw_token queue.
	///
	/// ## Examples
	/// ```
	/// use chearmyp_lexer::TokenQueue;
	///
	/// let token_queue = TokenQueue::new();
	/// ```
	pub fn new() -> Self {
		TokenQueue(VecDeque::new())
	}

	/// Pushes a raw_token to the raw_token queue.
	///
	/// ## Examples
	/// ```
	/// use chearmyp_lexer::RawToken;
	/// use chearmyp_lexer::TokenQueue;
	///
	/// let mut token_queue = TokenQueue::new();
	/// token_queue.push(RawToken::Simplex(b"hi"));
	/// ```
	pub fn push(&mut self, raw_token: RawToken<'a>) {
		self.0.push_back(raw_token);
	}

	/// Takes the oldest raw_token in the raw_token queue.
	///
	/// ## Examples
	/// ```
	/// use chearmyp_lexer::RawToken;
	/// use chearmyp_lexer::TokenQueue;
	///
	/// let mut token_queue = TokenQueue::new();
	/// token_queue.push(RawToken::Complex(b"universe"));
	/// token_queue.push(RawToken::Simplex(b"!"));
	///
	/// let oldest_token = token_queue.shift().unwrap();
	///
	/// assert_eq!(oldest_token, RawToken::Complex(b"universe"));
	/// ```
	pub fn shift(&mut self) -> Option<RawToken<'a>> {
		self.0.pop_front()
	}
}

mod from;
mod into_iterator;

#[cfg(test)]
use core::cmp::PartialEq;

#[cfg(test)]
impl<'a> PartialEq<VecDeque<RawToken<'a>>> for TokenQueue<'a> {
	fn eq(&self, other: &VecDeque<RawToken<'a>>) -> bool {
		PartialEq::eq(&self.0, other)
	}
}

#[cfg(test)]
mod t {
	use super::RawToken;
	use super::VecDeque;
	use super::TokenQueue;

	#[test]
	fn can_compare_to_itself() {
		let queue = TokenQueue::new();
		let other_queue = TokenQueue::new();
		assert_eq!(queue, other_queue);
	}

	#[test]
	fn can_push() {
		let raw_token = RawToken::Empty;
		let expected_token = RawToken::Empty;
		let mut queue = TokenQueue::new();
		let mut expected_queue = VecDeque::new();
		expected_queue.push_back(expected_token);

		queue.push(raw_token);

		assert_eq!(queue, expected_queue);
	}

	#[test]
	fn can_pop_on_empty_queue() {
		let mut queue = TokenQueue::new();
		let expected_token = None;

		let raw_token = queue.shift();

		assert_eq!(raw_token, expected_token);
	}

	#[test]
	fn can_pop_on_filled_queue() {
		let raw_token = RawToken::Empty;
		let expected_token = Some(RawToken::Empty);
		let mut queue = TokenQueue::new();
		queue.push(raw_token);

		let raw_token = queue.shift();

		assert_eq!(raw_token, expected_token);
	}
}
