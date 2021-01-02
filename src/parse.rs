mod node;

pub use node::Node;

use alloc::vec::Vec;
use crate::lex::{Token, TokenQueue};

pub fn parse<'a, T>(stream: T) -> Vec<Node<'a>>
where T: 'a + Into<TokenQueue<'a>> {
	let stream = stream.into();

	for token in stream {
		match token {
		}
	}

	unimplemented!()
}
