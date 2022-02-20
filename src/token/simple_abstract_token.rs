use crate::abstracts::{
	AbstractBoundary,
	SimpleAbstractToken,
	AbstractBoundaryCollection
};
use crate::token_kind::TokenKind;
use super::Token;

impl<T, U, V> SimpleAbstractToken<T, U, V> for Token<U, V>
where
	U: AbstractBoundary<T>,
	V: AbstractBoundaryCollection<T, U> {

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

	fn new_simplex(source: U) -> Self {
		Token::Simplex(source)
	}

	fn new_complex(source: U) -> Self {
		Token::Complex(source)
	}

	fn new_attacher(label: U, content: U) -> Self {
		Token::Attacher(label, content)
	}

	fn new_line_comment(line: U) -> Self {
		Token::LineComment(line)
	}

	fn new_block_comment(collection: V) -> Self {
		Token::BlockComment(collection)
	}

	fn new_line_othertongue(line: U) -> Self {
		Token::LineOthertongue(line)
	}

	fn new_block_othertongue(collection: V) -> Self {
		Token::BlockOthertongue(collection)
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{SimpleAbstractToken, Token, TokenKind};

	#[test]
	fn can_create_scope_level_and_confirm() {
		let scope_level = Token::<Range<u8>, Vec<Range<u8>>>::new_scope_level(0);

		let kind = scope_level.kind();

		assert_eq!(kind, TokenKind::ScopeLevel);
	}

	#[test]
	fn can_create_simplex_and_confirm() {
		let simplex = Token::<Range<u8>, Vec<Range<u8>>>::new_simplex(0..1);

		let kind = simplex.kind();

		assert_eq!(kind, TokenKind::Simplex);
	}

	#[test]
	fn can_create_complex_and_confirm() {
		let complex = Token::<Range<u8>, Vec<Range<u8>>>::new_complex(2..3);

		let kind = complex.kind();

		assert_eq!(kind, TokenKind::Complex);
	}

	#[test]
	fn can_create_attacher_and_confirm() {
		let attacher = Token::<Range<u8>, Vec<Range<u8>>>::new_attacher(4..5, 6..7);

		let kind = attacher.kind();

		assert_eq!(kind, TokenKind::Attacher);
	}

	#[test]
	fn can_create_line_comment_and_confirm() {
		let line_comment = Token::<Range<u8>, Vec<Range<u8>>>::new_line_comment(8..9);

		let kind = line_comment.kind();

		assert_eq!(kind, TokenKind::LineComment);
	}

	#[test]
	fn can_create_block_comment_and_confirm() {
		let block_comment = Token::<Range<u8>, Vec<Range<u8>>>::new_block_comment(Vec::new());

		let kind = block_comment.kind();

		assert_eq!(kind, TokenKind::BlockComment);
	}

	#[test]
	fn can_create_line_othertongue_and_confirm() {
		let line_othertongue = Token::<Range<u8>, Vec<Range<u8>>>::new_line_othertongue(10..11);

		let kind = line_othertongue.kind();

		assert_eq!(kind, TokenKind::LineOthertongue);
	}

	#[test]
	fn can_create_block_othertongue_and_confirm() {
		let block_othertongue = Token::<Range<u8>, Vec<Range<u8>>>
			::new_block_othertongue(Vec::new());

		let kind = block_othertongue.kind();

		assert_eq!(kind, TokenKind::BlockOthertongue);
	}
}
