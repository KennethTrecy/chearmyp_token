mod node;
mod scope_stack;

pub use node::Node;

use alloc::vec::Vec;
use crate::lex::{Token, TokenQueue};
use scope_stack::ScopeStack;

pub fn parse<'a, T>(stream: T) -> Vec<Node<'a>>
where T: 'a + Into<TokenQueue<'a>> {
	let stream = stream.into();
	let mut scope_stack = ScopeStack::new();

	for token in stream {
		match token {
			Token::Complex(concept) => scope_stack.append_complex(concept),
			Token::Attacher(label, content) => scope_stack.append_attacher(label, content),
			Token::Simplex(concept) => scope_stack.append_simplex(concept),
			Token::ScopeLevel(level) => scope_stack.minimize_scope_level_by(level),
			Token::LineComment(comment) => scope_stack.append_line_comment(comment),
			Token::BlockComment(comment_lines) => scope_stack.append_block_comment(comment_lines),
			Token::LineOthertongue(othertongue) => scope_stack.append_line_othertongue(othertongue),
			Token::BlockOthertongue(othertongue_lines) => {
				scope_stack.append_block_othertongue(othertongue_lines);
			},
			_ => unimplemented!()
		}
	}

	scope_stack.finalize()
}
