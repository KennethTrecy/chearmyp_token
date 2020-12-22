use alloc::collections::VecDeque;
use crate::lex::Token;

/// Contains the tokens that have been lexed.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct TokenQueue<'a>(pub VecDeque<Token<'a>>);

impl<'a> TokenQueue<'a> {
	/// Creates the token queue.
	///
	/// ## Examples
	/// ```
	/// use chearmyp::lex::TokenQueue;
	///
	/// let token_queue = TokenQueue::new();
	/// ```
	pub fn new() -> Self {
		TokenQueue(VecDeque::new())
	}

	/// Pushes a token to the token queue.
	///
	/// ## Examples
	/// ```
	/// use chearmyp::lex::Token;
	/// use chearmyp::lex::TokenQueue;
	///
	/// let mut token_queue = TokenQueue::new();
	/// token_queue.push(Token::Simplex(b"hi"));
	/// ```
	pub fn push(&mut self, token: Token<'a>) {
		self.0.push_back(token);
	}

	/// Takes the oldest token in the token queue.
	///
	/// ## Examples
	/// ```
	/// use chearmyp::lex::Token;
	/// use chearmyp::lex::TokenQueue;
	///
	/// let mut token_queue = TokenQueue::new();
	/// token_queue.push(Token::Complex(b"universe"));
	/// token_queue.push(Token::Simplex(b"!"));
	///
	/// let oldest_token = token_queue.shift().unwrap();
	///
	/// assert_eq!(oldest_token, Token::Complex(b"universe"));
	/// ```
	pub fn shift(&mut self) -> Option<Token<'a>> {
		self.0.pop_front()
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
	use super::Token;
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
		let token = Token::Empty;
		let expected_token = Token::Empty;
		let mut queue = TokenQueue::new();
		let mut expected_queue = VecDeque::new();
		expected_queue.push_back(expected_token);

		queue.push(token);

		assert_eq!(queue, expected_queue);
	}

	#[test]
	fn can_pop_on_empty_queue() {
		let mut queue = TokenQueue::new();
		let expected_token = None;

		let token = queue.shift();

		assert_eq!(token, expected_token);
	}

	#[test]
	fn can_pop_on_filled_queue() {
		let token = Token::Empty;
		let expected_token = Some(Token::Empty);
		let mut queue = TokenQueue::new();
		queue.push(token);

		let token = queue.shift();

		assert_eq!(token, expected_token);
	}
}
