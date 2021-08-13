use crate::abstracts::{ AbstractSource, AbstractLineCommentToken };
use crate::Token;

impl<T, U> AbstractLineCommentToken for Token<T, U>
where
	T: AbstractSource {
	type Line = T::Slice;

	fn line(&self) -> Self::Line {
		match self {
			Self::LineComment(line) => line.forward_slice(0),
			_ => panic!("Cannot retrieve the line in the line comment token")
		}
	}
}

#[cfg(test)]
mod t {
	use super::{ AbstractLineCommentToken, Token };

	#[test]
	fn can_get_line() {
		let line_comment = Token::<&str, &str>::LineComment("a");

		let line = line_comment.line();

		assert_eq!(line, "a");
	}

	#[test]
	#[should_panic]
	fn cannot_get_line() {
		let simplex = Token::<&str, &str>::Simplex("b");

		simplex.line();
	}
}
