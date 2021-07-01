use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{EQUAL, POUND_SIGN, SPACE};
use crate::{
	simplex,
	complex,
	attacher,
	line_comment,
	block_comment,
	line_othertongue,
	block_othertongue};

/// Returns the info of first recognized raw_token and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the raw_token as the second argument (known as the offset), and the number of tabs to work in case
/// it found a block raw_token of any kind.
///
/// ## Notes
/// May panic if the last possible lexer has returned an unexpected raw_token.
///
/// ## Examples
/// ```
/// use chearmyp_lexer::any;
/// use chearmyp_lexer::RawToken;
///
/// let (any, last_index) = any(&b"hello"[..], 0, 0);
/// if let RawToken::Complex(content) = any {
/// 	assert_eq!(content, &b"hello"[..]);
/// } else {
/// 	panic!("The returned raw_token is not complex.");
/// }
/// assert_eq!(last_index, 5);
/// ```
pub fn any(src: &[u8], offset: usize, tab_count: usize) -> RawTokenInfo {
	let original_offset = offset;
	let mut raw_token;
	let mut offset = offset;

	macro_rules! lex {
		(
			$parser:ident$(($($other_argument:tt),+))?
			$(unless $raw_token:ident($($content:tt),+) => $block:block)?
			$(which expects $expected_token:ident($($expected_content:tt),+))?
		) => {
			let info = $parser(src, offset, $($($other_argument,)*)?);
			raw_token = info.0;
			offset = info.1;
			$(
				if let RawToken::$raw_token($($content,)+) = raw_token {
					(raw_token, offset)
				} else $block
			)?
			$(
				if let RawToken::$expected_token($($expected_content,)+) = raw_token {
					(raw_token, offset)
				} else {
					panic!("There is an unxpected raw_token in lexing the source.");
				}
			)?
		};
	}

	if src[0] == POUND_SIGN {
		lex!{
			block_comment(tab_count)
			unless BlockComment(_) => {
				lex!{ line_comment which expects LineComment(_) }
			}
		}
	} else if src[0] == EQUAL || (src[0] == SPACE && src[1] == EQUAL) {
		lex!{
			block_othertongue(tab_count)
			unless BlockOthertongue(_) => {
				lex!{ line_othertongue which expects LineOthertongue(_) }
			}
		}
	} else {
		lex!{
			attacher(0)
			unless Attacher(_, _) => {
				let search_offset = if offset > 0 { offset - 1 } else { 0 };
				offset = 0;
				lex!{
					simplex(search_offset)
					unless Simplex(_) => {
						if offset > 0 { offset -= 1; }
						let search_offset = offset;
						offset = original_offset;
						lex!{ complex(search_offset) which expects Complex(_) }
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use super::{RawToken, any};

	macro_rules! test_any {
		($sample:literal $expected_info:expr) => {
			test_any!{
				sample: $sample,
				tab_count: 0,
				info: $expected_info
			}
		};
		(
			sample: $sample:expr,
			tab_count: $tab_count:literal,
			info: $expected_info:expr
		) => {
			let info = any(
				$sample,
				0,
				$tab_count
			);
			assert_eq!(info, $expected_info);
		};
	}

	#[test]
	fn can_lex_line_comment() {
		test_any!(b"#abc" (RawToken::LineComment(b"abc"), 4));
	}

	#[test]
	fn can_lex_block_comment() {
		let mut expected_lines = Vec::new();
		expected_lines.push(&b"\tde"[..]);

		test_any!(b"###\n\tde\n###" (RawToken::BlockComment(expected_lines), 11));
	}

	#[test]
	fn can_lex_simplex() {
		test_any!(b"efg|" (RawToken::Simplex(b"efg"), 4));
	}

	#[test]
	fn can_lex_complex() {
		test_any!(b"hi" (RawToken::Complex(b"hi"), 2));
	}

	#[test]
	fn can_lex_attacher() {
		test_any!(b"jklm:\tn" (RawToken::Attacher(b"jklm", b"n"), 7));
	}

	#[test]
	fn can_lex_line_othertongue() {
		test_any!(b"= o" (RawToken::LineOthertongue(b"o"), 3));
	}

	#[test]
	fn can_lex_block_othertongue() {
		let mut expected_lines = Vec::new();
		expected_lines.push(&b"pqrs"[..]);

		test_any!(b"===\npqrs\n===" (RawToken::BlockOthertongue(expected_lines), 12));
	}

	#[test]
	fn can_lex_inlined_line_othertongue() {
		test_any!(b" = tu" (RawToken::LineOthertongue(b"tu"), 5));
	}
}
