use alloc::vec::Vec;
use crate::find_line_ending;
use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, TAB};

pub fn block(src: &[u8], offset: usize, tab_count: usize, special_character: u8) -> TokenInfo {
	if !has_3_special_characters(src, offset, special_character) {
		(Token::Invalid, offset);
	}

	let mut lines = Vec::new();
	let mut offset = if let Some(&NEW_LINE) = src.get(offset + 3) { 4 } else { 3 };

	loop {
		let start = offset;
		let end = find_line_ending(src, start);
		if start == end { break; }
		let line = &src[start..end];

		let mut indent_size = tab_count;
		while indent_size > 0 {
			indent_size -= 1;
			if line[indent_size] != TAB { break; }
		}

		offset = end;

		if indent_size == 0 && has_3_special_characters(line, tab_count, special_character) {
			if let Some(&NEW_LINE) = src.get(offset) { offset += 1; }
			break;
		}

		offset += 1;
		lines.push(line);
	}

	(Token::Block(lines), offset)
}

fn has_3_special_characters(src: &[u8], offset: usize, special_character: u8) -> bool {
	if let Some(_) = src.get(offset + 2) {
		src[offset] == special_character
		&& src[offset + 1] == special_character
		&& src[offset + 2] == special_character
	} else {
		false
	}
}
