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
