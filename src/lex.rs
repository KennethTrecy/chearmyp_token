/// Contains the data structure returned by the main Lexer.
mod token_queue;

/// Contains the data structures and type aliases used and/or returned by different lexers.
mod token;

/// Contains different characters needed to be recognized by the different lexers.
pub mod special_characters;

/// Contains `find_line_ending()`.
mod find_line_ending;

/// Contains `line_comment()` lexer.
mod line_comment;

/// Contains `block_comment()` lexer.
mod block_comment;

/// Contains `simplex()` lexer.
mod simplex;

/// Contains `complex()` lexer and `determine_ending()`.
mod complex;

/// Contains `attacher()` lexer.
mod attacher;

/// Contains `line_othertongue()` lexer.
mod line_othertongue;

/// Contains `block()` lexer.
mod block;

/// Contains `block_othertongue()` lexer.
mod block_othertongue;

/// Contains types of delimeter that lexers search for.
mod delimeter;

// Contains `any()` lexer.
mod any;

// Contains `count_tabs()` counter.
mod count_tabs;

pub use block::block;
pub use find_line_ending::find_line_ending;
pub use line_comment::line_comment;
pub use block_comment::block_comment;
pub use simplex::simplex;
pub use complex::complex;
pub use attacher::attacher;
pub use line_othertongue::line_othertongue;
pub use block_othertongue::block_othertongue;
pub use any::any;
pub use token::Token;
pub use token_queue::TokenQueue;

use alloc::collections::VecDeque;
use special_characters::NEW_LINE;
use count_tabs::count_tabs;

/// Returns a stream of tokens based from the source.
///
/// The source is the first argument which contain an array of bytes.
///
/// ## Examples
/// ```
/// use std::collections::VecDeque;
/// use chearmyp::lex::lex;
/// use chearmyp::lex::{Token, TokenQueue};
/// let source = b"
/// a complex
/// this:	is an attacher
/// 	a simplex|
/// ## This is a line comment
/// ";
///
/// let queue: TokenQueue = lex(&source[..]);
/// let queue: VecDeque<Token> = queue.0;
///
/// assert_eq!(queue[0], Token::Complex(b"a complex"));
/// assert_eq!(queue[1], Token::Attacher(b"this", b"is an attacher"));
/// assert_eq!(queue[2], Token::ScopeLevel(1));
/// assert_eq!(queue[3], Token::Simplex(b"a simplex"));
/// assert_eq!(queue[4], Token::ScopeLevel(0));
/// assert_eq!(queue[5], Token::LineComment(b" This is a line comment"));
/// ```
pub fn lex(mut src: &[u8]) -> TokenQueue {
	let mut token_queue = VecDeque::new();
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
			token_queue.push_back(Token::ScopeLevel(new_tab_count));
			tab_count = new_tab_count;
		}

		let (token, size) = any(src, 0, tab_count);
		token_queue.push_back(token);

		scanned_size += size;
		src = &src[size..];
	}

	TokenQueue(token_queue)
}
