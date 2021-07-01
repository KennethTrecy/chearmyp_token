use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{EQUAL, SPACE};
use crate::delimeter::Delimeter;
use crate::find_line_ending;

/// Returns the info of recognized line othertogue and the probably last index that has been checked
/// from the source.
///
/// It needs an array of bytes as the first argument (known as source), and where to start looking
/// for the line othertongue (inlined or not) as the second argument (known as the offset).
///
/// ## Notes
/// If there is no valid raw token found, it will return invalid raw token along with the probably
/// last index checked.
///
/// ## Examples
/// ```
/// use chearmyp_lexer::line_othertongue;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"= hello world";
/// let (raw_token, last_index) = line_othertongue(&non_terminated[..], 0);
/// if let RawToken::LineOthertongue(raw_token) = raw_token {
/// 	assert_eq!(raw_token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned raw token is not line othertongue.");
/// }
/// assert_eq!(last_index, 13);
///
/// let inlined_yet_terminated = b" = hello world\n";
/// let (raw_token, last_index) = line_othertongue(&inlined_yet_terminated[..], 0);
/// if let RawToken::LineOthertongue(raw_token) = raw_token {
/// 	assert_eq!(raw_token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned raw token is not line othertongue.");
/// }
/// assert_eq!(last_index, 14);
/// ```
pub fn line_othertongue(src: &[u8], offset: usize) -> RawTokenInfo {
	match determine_othertongue_prefix(src, offset) {
		Delimeter::Pad => {
			let start = offset + if src[offset] == EQUAL { 2 } else { 3 };
			let end = find_line_ending(src, start);
			(RawToken::LineOthertongue(&src[start..end]), end)
		},
		_ => (RawToken::Invalid, offset)
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
	use super::{RawToken, line_othertongue};

	macro_rules! test_line_othertongue {
		($sample:literal 0 $variant:ident) => {
			let (raw_token, last_seen_offset) = line_othertongue($sample, 0);
			assert_eq!(last_seen_offset, 0, "Expected raw_token of {:?}", $sample);
			assert_eq!(raw_token, RawToken::$variant, "Expected last seen offset of {:?}", $sample);
		};
		($sample:literal $expected_offset:literal $expected_token:expr) => {
			let (raw_token, last_seen_offset) = line_othertongue($sample, 0);
			assert_eq!(raw_token, RawToken::LineOthertongue(&$expected_token[..]),
				"Expected raw_token of {:?}", $sample);
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
