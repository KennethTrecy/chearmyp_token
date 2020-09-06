use crate::token::{Token, TokenInfo};
use crate::special_characters::{EQUAL, SPACE};
use crate::delimeter::Delimeter;
use crate::find_line_ending;

pub fn line_othertongue(src: &[u8], i: usize) -> TokenInfo {
	match determine_othertongue_prefix(src, i) {
		Delimeter::Pad => {
			let start = i + if src[i] == EQUAL { 2 } else { 3 };
			let end = find_line_ending(src, start);
			(Token::LineOthertongue(&src[start..end]), end)
		},
		_ => (Token::Invalid, i)
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
