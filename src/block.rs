use alloc::vec::Vec;
use crate::find_line_ending;
use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, TAB};

pub fn block(src: &[u8], i: usize, tab_count: usize, special_character: u8) -> TokenInfo {
	if !has_3_special_characters(src, i, special_character) {
		return (Token::Invalid, i);
	}

	let mut lines = Vec::new();
	let limit = src.len();
	let mut i = if src[i + 3] == NEW_LINE { 4 } else { 3 };

	while i < limit {
		let start = i;
		let end = find_line_ending(src, start);
		let line = &src[start..end];

		let mut indent_size = tab_count;
		while indent_size > 0 {
			indent_size -= 1;
			if line[indent_size] != TAB { break; }
		}

		i = end;

		if indent_size == 0 && has_3_special_characters(line, tab_count, special_character) {
			if i + 1 < limit && src[i] == NEW_LINE { i += 1; }
			break;
		}

		i += 1;
		lines.push(line);
	}

	(Token::Block(lines), i)
}

fn has_3_special_characters(src: &[u8], i: usize, special_character: u8) -> bool {
	return src[i] == special_character
		&& src[i + 1] == special_character
		&& src[i + 2] == special_character;
}
