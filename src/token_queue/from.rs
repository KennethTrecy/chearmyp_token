use core::convert::From;
use super::TokenQueue;
use crate::lex;

impl<'a> From<&'a [u8]> for TokenQueue<'a> {
	fn from(src: &'a [u8]) -> Self {
		lex(&src[..])
	}
}

#[cfg(test)]
mod t {
	use crate::Token;
	use super::TokenQueue;

	#[test]
	fn can_convert_from_u8_slice() {
		let u8_slice = &b"# abcd"[..];
		let expected_token = Token::LineComment(b" abcd");
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(expected_token);

		let token_queue: TokenQueue = u8_slice.into();

		assert_eq!(token_queue, expected_token_queue);
	}
}
