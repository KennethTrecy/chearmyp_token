use crate::{any, RawToken, TokenQueue};
use crate::count_tabs::count_tabs;
use crate::special_characters::NEW_LINE;

/// Returns a stream of tokens based from the source.
///
/// The source is the first argument which contain an array of bytes. This is the main lexer.
///
/// ## Examples
/// ```
/// use std::collections::VecDeque;
/// use chearmyp_lexer::lex;
/// use chearmyp_lexer::{RawToken, TokenQueue};
/// let source = b"
/// a complex
/// 	this:	is an attacher
/// 	a simplex|
/// ## This is a line comment
/// ";
///
/// let queue: TokenQueue = lex(&source[..]);
/// let queue: VecDeque<RawToken> = queue.0;
///
/// assert_eq!(queue[0], RawToken::Complex(b"a complex"));
/// assert_eq!(queue[1], RawToken::ScopeLevel(1));
/// assert_eq!(queue[2], RawToken::Attacher(b"this", b"is an attacher"));
/// assert_eq!(queue[3], RawToken::Simplex(b"a simplex"));
/// assert_eq!(queue[4], RawToken::ScopeLevel(0));
/// assert_eq!(queue[5], RawToken::LineComment(b" This is a line comment"));
/// ```
pub fn lex(mut src: &[u8]) -> TokenQueue {
	let mut token_queue = TokenQueue::new();
	let mut tab_count = 0;
	let mut scanned_size = 0;
	let limit = src.len();

	while scanned_size < limit {
		if src[0] == NEW_LINE {
			scanned_size += 1;
			src = &src[1..];
			continue;
		}

		src = &src[0..];
		let new_tab_count = count_tabs(src, tab_count);

		scanned_size += new_tab_count;
		src = &src[new_tab_count..];

		if new_tab_count != tab_count {
			token_queue.push(RawToken::ScopeLevel(new_tab_count));
			tab_count = new_tab_count;
		}

		let (token, size) = any(src, 0, tab_count);
		token_queue.push(token);

		scanned_size += size;
		src = &src[size..];
	}

	token_queue
}


#[cfg(test)]
mod t {
	use alloc::vec;
	use super::lex;
	use super::{RawToken, TokenQueue};

	#[test]
	fn can_lex_line_comment() {
		let source = b"# Hello World";
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(RawToken::LineComment(&source[1..]));

		let token_queue = lex(&source[..]);

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_block_comment() {
		let source = b"###\nHello world\n###";
		let first_index_of_hello_world = 4;
		let last_index_of_hello_world = source.len() - 4;
		let expected_lines = vec![&source[first_index_of_hello_world..last_index_of_hello_world]];
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(RawToken::BlockComment(expected_lines));

		let token_queue = lex(&source[..]);

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_simplex() {
		let source = b"hello_world|";
		let last_index = source.len() - 1;
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(RawToken::Simplex(&source[0..last_index]));

		let token_queue = lex(&source[..]);

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_complex() {
		let source = b"HelloWorld";
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(RawToken::Complex(&source[..]));

		let token_queue = lex(&source[..]);

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_attacher() {
		let source = b"hello:	world";
		let expected_label = b"hello";
		let expected_content = b"world";
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(RawToken::Attacher(&expected_label[..], &expected_content[..]));

		let token_queue = lex(&source[..]);

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_line_othertongue() {
		let source = b" = hello-world";
		let first_index_of_hello_world = 3;
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(RawToken::LineOthertongue(&source[first_index_of_hello_world..]));

		let token_queue = lex(&source[..]);

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_block_othertongue() {
		let source = b"===\nhelloWorld\n===";
		let first_index_of_hello_world = 4;
		let last_index_of_hello_world = source.len() - 4;
		let expected_lines = vec![&source[first_index_of_hello_world..last_index_of_hello_world]];
		let mut expected_token_queue = TokenQueue::new();
		expected_token_queue.push(RawToken::BlockOthertongue(expected_lines));

		let token_queue = lex(&source[..]);

		assert_eq!(token_queue, expected_token_queue);
	}
}
