use crate::token::{Token, TokenInfo};
use crate::special_characters::{EQUAL, SPACE};
use crate::delimeter::Delimeter;
use crate::find_line_ending;

pub fn line_othertongue(src: &[u8], i: usize) -> TokenInfo {
	match determine_othertongue_prefix(src, i) {
		Delimeter::Pad => {
			let start = i + if src[i] == EQUAL { 2 } else { 3 };
			let limit = src.len();
			let end = find_line_ending(src, start, limit);
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
	|| (first_character == space && second_character == equal && src.get(offset + 3) == space) {
		Delimeter::Pad
	} else {
		Delimeter::Invalid
	}
}
