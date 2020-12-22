use crate::lex::token::{Token, TokenInfo};
use crate::lex::special_characters::{EQUAL, SPACE};
use crate::lex::delimeter::Delimeter;
use crate::lex::find_line_ending;

/// Returns the info of recognized line othertogue and the probably last index that has been checked
/// from the source.
///
/// It needs an array of bytes as the first argument (known as source), and where to start looking
/// for the line othertongue (inlined or not) as the second argument (known as the offset).
///
/// ## Notes
/// If there is no valid token found, it will return invalid token along with the probably last
/// index checked.
///
/// ## Examples
/// ```
/// use chearmyp::lex::line_othertongue;
/// use chearmyp::lex::Token;
///
/// let non_terminated = b"= hello world";
/// let (token, last_index) = line_othertongue(&non_terminated[..], 0);
/// if let Token::LineOthertongue(token) = token {
/// 	assert_eq!(token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned token is not line othertongue.");
/// }
/// assert_eq!(last_index, 13);
///
/// let inlined_yet_terminated = b" = hello world\n";
/// let (token, last_index) = line_othertongue(&inlined_yet_terminated[..], 0);
/// if let Token::LineOthertongue(token) = token {
/// 	assert_eq!(token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned token is not line othertongue.");
/// }
/// assert_eq!(last_index, 14);
/// ```
pub fn line_othertongue(src: &[u8], offset: usize) -> TokenInfo {
	match determine_othertongue_prefix(src, offset) {
		Delimeter::Pad => {
			let start = offset + if src[offset] == EQUAL { 2 } else { 3 };
			let end = find_line_ending(src, start);
			(Token::LineOthertongue(&src[start..end]), end)
		},
		_ => (Token::Invalid, offset)
	}
}

pub fn determine_othertongue_prefix(src: &[u8], offset: usize) -> Delimeter {
	let first_character = src.get(offset);
	let second_character = src.get(offset + 1);
	let equal = Some(&EQUAL);
	let space = Some(&SPACE);
	if (first_character == equal && second_character == space)
	|| (first_character == space && second_character == equal && src.get(offset + 2) == space) {
		Delimeter::Pad
	} else {
		Delimeter::Invalid
	}
}

#[cfg(test)]
mod t {
	use super::{Token, line_othertongue};

	macro_rules! test_line_othertongue {
		($sample:literal 0 $variant:ident) => {
			let (token, last_seen_offset) = line_othertongue($sample, 0);
			assert_eq!(last_seen_offset, 0, "Expected token of {:?}", $sample);
			assert_eq!(token, Token::$variant, "Expected last seen offset of {:?}", $sample);
		};
		($sample:literal $expected_offset:literal $expected_token:expr) => {
			let (token, last_seen_offset) = line_othertongue($sample, 0);
			assert_eq!(token, Token::LineOthertongue(&$expected_token[..]),
				"Expected token of {:?}", $sample);
			assert_eq!(last_seen_offset, $expected_offset,
				"Expected last seen offset of {:?}", $sample);
		};
	}

	#[test]
	fn can_lex() {
		test_line_othertongue!(b"= a" 3 b"a");
		test_line_othertongue!(b" = bc" 5 b"bc");
	}

	#[test]
	fn cannot_lex() {
		test_line_othertongue!(b"=d" 0 Invalid);
		test_line_othertongue!(b" =e" 0 Invalid);
		test_line_othertongue!(b"f" 0 Invalid);
	}
}
