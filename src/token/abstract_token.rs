use crate::abstracts::{ AbstractSource, AbstractSourceCollection, AbstractToken };
use crate::token_kind::TokenKind;
use crate::Token;

impl<T, U> AbstractToken for Token<T, U>
where
	T: AbstractSource,
	U: AbstractSourceCollection {
	type Source = T;
	type SourceCollection = U;

	fn kind(&self) -> TokenKind {
		match self {
			Self::ScopeLevel(_) => TokenKind::ScopeLevel,
			Self::Simplex(_) => TokenKind::Simplex,
			Self::Complex(_) => TokenKind::Complex,
			Self::Attacher(_, _) => TokenKind::Attacher,
			Self::LineComment(_) => TokenKind::LineComment,
			Self::BlockComment(_) => TokenKind::BlockComment,
			Self::LineOthertongue(_) => TokenKind::LineOthertongue,
			Self::BlockOthertongue(_) => TokenKind::BlockOthertongue
		}
	}

	fn new_scope_level(level: usize) -> Self {
		Token::ScopeLevel(level)
	}

	fn new_simplex(source: Self::Source) -> Self {
		Token::Simplex(source)
	}

	fn new_complex(source: Self::Source) -> Self {
		Token::Complex(source)
	}

	fn new_attacher(label: Self::Source, content: Self::Source) -> Self {
		Token::Attacher(label, content)
	}

	fn new_line_comment(line: Self::Source) -> Self {
		Token::LineComment(line)
	}

	fn new_block_comment(collection: Self::SourceCollection) -> Self {
		Token::BlockComment(collection)
	}

	fn new_line_othertongue(line: Self::Source) -> Self {
		Token::LineOthertongue(line)
	}

	fn new_block_othertongue(collection: Self::SourceCollection) -> Self {
		Token::BlockOthertongue(collection)
	}
}

#[cfg(test)]
mod t {
	#[cfg(feature = "no_std")]
	use alloc::vec::Vec;

	use super::{ AbstractToken, Token, TokenKind };

	#[test]
	fn can_create_scope_level_and_confirm() {
		let scope_level = Token::<&str, Vec<&str>>::new_scope_level(0);

		let kind = scope_level.kind();

		assert_eq!(kind, TokenKind::ScopeLevel);
	}

	#[test]
	fn can_create_simplex_and_confirm() {
		let simplex = Token::<&str, Vec<&str>>::new_simplex("a");

		let kind = simplex.kind();

		assert_eq!(kind, TokenKind::Simplex);
	}

	#[test]
	fn can_create_complex_and_confirm() {
		let complex = Token::<&str, Vec<&str>>::new_complex("b");

		let kind = complex.kind();

		assert_eq!(kind, TokenKind::Complex);
	}

	#[test]
	fn can_create_attacher_and_confirm() {
		let attacher = Token::<&str, Vec<&str>>::new_attacher("c", "d");

		let kind = attacher.kind();

		assert_eq!(kind, TokenKind::Attacher);
	}

	#[test]
	fn can_create_line_comment_and_confirm() {
		let line_comment = Token::<&str, Vec<&str>>::new_line_comment("e");

		let kind = line_comment.kind();

		assert_eq!(kind, TokenKind::LineComment);
	}

	#[test]
	fn can_create_block_comment_and_confirm() {
		let block_comment = Token::<&str, Vec<&str>>::new_block_comment(Vec::new());

		let kind = block_comment.kind();

		assert_eq!(kind, TokenKind::BlockComment);
	}

	#[test]
	fn can_create_line_othertongue_and_confirm() {
		let line_othertongue = Token::<&str, Vec<&str>>::new_line_othertongue("e");

		let kind = line_othertongue.kind();

		assert_eq!(kind, TokenKind::LineOthertongue);
	}

	#[test]
	fn can_create_block_othertongue_and_confirm() {
		let block_othertongue = Token::<&str, Vec<&str>>::new_block_othertongue(Vec::new());

		let kind = block_othertongue.kind();

		assert_eq!(kind, TokenKind::BlockOthertongue);
	}
}
