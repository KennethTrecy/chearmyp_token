use crate::abstracts::DynamicAbstractToken;
use super::Token;

impl<T, U> DynamicAbstractToken for Token<T, U> {
	type Name = T;
	type Line = T;
	type Block = U;
	type Label = T;
	type Content = T;

	fn name(&self) -> &Self::Name {
		match self {
			Self::Simplex(name) | Self::Complex(name) => name,
			_ => panic!("Cannot retrieve the name because it is not a simplex or complex token.")
		}
	}

	fn line(&self) -> &Self::Line {
		match self {
			Self::LineComment(line) | Self::LineOthertongue(line) => line,
			_ => panic!("Cannot retrieve the line because it is not a line comment/othertongue token.")
		}
	}

	fn block(&self) -> &Self::Block {
		match self {
			Self::BlockComment(block) | Self::BlockOthertongue(block) => block,
			_ => panic!("Cannot retrieve the block because it is not a block comment/othertongue token.")
		}
	}

	fn level(&self) -> usize {
		match self {
			Self::ScopeLevel(level) => *level,
			_ => panic!("Cannot retrieve the scope level because it is not a scope level token.")
		}
	}

	fn label(&self) -> &Self::Label {
		match self {
			Self::Attacher(label, _) => label,
			_ => panic!("Cannot retrieve the label because it is not an attacher token.")
		}
	}

	fn content(&self) -> &Self::Content {
		match self {
			Self::Attacher(_, content) => content,
			_ => panic!("Cannot retrieve the content because it is not an attacher token.")
		}
	}

	fn consume_attacher(self) -> (Self::Label, Self::Content) {
		match self {
			Self::Attacher(label, content) => (label, content),
			_ => panic!("Cannot consume because it is not an attacher token.")
		}
	}

	fn consume_block(self) -> Self::Block {
		match self {
			Self::BlockComment(block) | Self::BlockOthertongue(block) => block,
			_ => panic!("Cannot consume because it is not a block comment/othertongue token.")
		}
	}

	fn consume_concept(self) -> Self::Name {
		match self {
			Self::Simplex(name) | Self::Complex(name) => name,
			_ => panic!("Cannot consume because it is not a simplex or complex token.")
		}
	}

	fn consume_line(self) -> Self::Line {
		match self {
			Self::LineComment(line) | Self::LineOthertongue(line) => line,
			_ => panic!("Cannot consume because it is not a line comment/othertongue token.")
		}
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use crate::abstracts::SimpleAbstractToken;
	use super::Token;
	use crate::abstracts::AbstractAttacherToken;

	#[test]
	fn can_get_label() {
		let attacher = Token::<Range<u8>, Vec<Range<u8>>>::new_attacher(0..1, 2..3);

		let label = attacher.label();

		assert_eq!(label, &(0..1));
	}

	#[test]
	fn can_get_content() {
		use crate::abstracts::AbstractAttacherToken;

		let attacher = Token::<Range<u8>, Vec<Range<u8>>>::new_attacher(4..5, 6..7);

		let content = attacher.content();

		assert_eq!(content, &(6..7));
	}

	#[test]
	#[should_panic]
	fn cannot_get_label() {
		use crate::abstracts::AbstractAttacherToken;

		let line_othertongue = Token::<Range<u8>, Vec<Range<u8>>>::new_line_othertongue(8..9);

		line_othertongue.label();
	}

	#[test]
	#[should_panic]
	fn cannot_get_content() {
		use crate::abstracts::AbstractAttacherToken;

		let line_comment = Token::<Range<u8>, Vec<Range<u8>>>::new_line_comment(10..11);

		line_comment.content();
	}

	#[test]
	fn can_get_block_comment() {
		use crate::abstracts::AbstractBlockCommentToken;

		let block_comment = Token::<Range<u8>, Vec<Range<u8>>>::new_block_comment(vec![ 12..13 ]);

		let block = block_comment.block();

		assert_eq!(block, &vec![ 12..13 ]);
	}

	#[test]
	#[should_panic]
	fn cannot_get_block_comment() {
		use crate::abstracts::AbstractBlockCommentToken;

		let simplex = Token::<Range<u8>, Vec<Range<u8>>>::new_simplex(14..15);

		simplex.block();
	}

	#[test]
	fn can_get_block_othertongue() {
		use crate::abstracts::AbstractBlockOthertongueToken;

		let block_othertongue = Token::<Range<u8>, Vec<Range<u8>>>
			::new_block_othertongue(vec![ 16..17 ]);

		let block = block_othertongue.block();

		assert_eq!(block, &vec![ 16..17 ]);
	}

	#[test]
	#[should_panic]
	fn cannot_get_block_othertongue() {
		use crate::abstracts::AbstractBlockOthertongueToken;

		let simplex = Token::<Range<u8>, Vec<Range<u8>>>::new_simplex(18..19);

		simplex.block();
	}

	#[test]
	fn can_get_line_comment() {
		use crate::abstracts::AbstractLineCommentToken;

		let line_comment = Token::<Range<u8>, Vec<Range<u8>>>::new_line_comment(20..21);

		let line = line_comment.line();

		assert_eq!(line, &(20..21));
	}

	#[test]
	#[should_panic]
	fn cannot_get_line_comment() {
		use crate::abstracts::AbstractLineCommentToken;

		let complex = Token::<Range<u8>, Vec<Range<u8>>>::new_complex(22..23);

		complex.line();
	}

	#[test]
	fn can_get_line_othertongue() {
		use crate::abstracts::AbstractLineOthertongueToken;

		let line_othertongue = Token::<Range<u8>, Vec<Range<u8>>>::new_line_othertongue(24..25);

		let line = line_othertongue.line();

		assert_eq!(line, &(24..25));
	}

	#[test]
	#[should_panic]
	fn cannot_get_line_othertongue() {
		use crate::abstracts::AbstractLineOthertongueToken;

		let simplex = Token::<Range<u8>, Vec<Range<u8>>>::new_simplex(26..27);

		simplex.line();
	}

	#[test]
	fn can_get_level() {
		use crate::abstracts::AbstractScopeLevelToken;

		let scope_level = Token::<Range<u8>, Vec<Range<u8>>>::new_scope_level(1);

		let level = scope_level.level();

		assert_eq!(level, 1);
	}

	#[test]
	#[should_panic]
	fn cannot_get_level() {
		use crate::abstracts::AbstractScopeLevelToken;

		let line_othertongue = Token::<Range<u8>, Vec<Range<u8>>>::new_line_othertongue(28..29);

		line_othertongue.level();
	}

	#[test]
	fn can_get_complex_name() {
		use crate::abstracts::AbstractComplexToken;

		let complex = Token::<Range<u8>, Vec<Range<u8>>>::new_complex(30..31);

		let name = complex.name();

		assert_eq!(name, &(30..31));
	}

	#[test]
	#[should_panic]
	fn cannot_get_complex_name() {
		use crate::abstracts::AbstractComplexToken;

		let line_comment = Token::<Range<u8>, Vec<Range<u8>>>::new_line_comment(32..33);

		line_comment.name();
	}

	#[test]
	fn can_get_simplex_name() {
		use crate::abstracts::AbstractSimplexToken;

		let simplex = Token::<Range<u8>, Vec<Range<u8>>>::new_simplex(34..35);

		let name = simplex.name();

		assert_eq!(name, &(34..35));
	}

	#[test]
	#[should_panic]
	fn cannot_get_simplex_name() {
		use crate::abstracts::AbstractSimplexToken;

		let line_comment = Token::<Range<u8>, Vec<Range<u8>>>::new_line_comment(36..37);

		line_comment.name();
	}

	#[test]
	fn can_consume_attacher() {
		use crate::abstracts::AbstractAttacherToken;

		let attacher = Token::<Range<u8>, Vec<Range<u8>>>::new_attacher(0..1, 2..3);

		let (label, content) = attacher.consume();

		assert_eq!(label, 0..1);
		assert_eq!(content, 2..3);
	}
}
