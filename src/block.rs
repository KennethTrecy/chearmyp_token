use alloc::vec::Vec;
use crate::find_line_ending;
use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, TAB};

pub fn block(src: &[u8], offset: usize, tab_count: usize, special_character: u8) -> TokenInfo {
	if has_3_special_characters(src, offset, special_character) {
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
	} else {
		let token = if let Some(_) = src.get(offset) { Token::Invalid } else { Token::Empty };
		(token, offset)
	}
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

#[cfg(test)]
mod t {
	use super::{has_3_special_characters, block, Token};

	macro_rules! has_3_special_characters {
		($src:literal $offset:literal $special_character:literal) => {
			has_3_special_characters(&$src[..], $offset, $special_character as u8)
		};
	}

	macro_rules! block {
		($src:literal $offset:literal $tab_count:literal $special_character:literal) => {
			block(&$src[..], $offset, $tab_count, $special_character as u8)
		};
	}

	macro_rules! Block {
		($($token:literal)*) => {
			Token::Block(alloc::vec![$(&$token[..],)*])
		};
	}

	#[test]
	fn can_detect_special_characters() {
		assert!(has_3_special_characters!(b"aaa" 0 'a'), "Normal string");
	}

	#[test]
	fn cannot_detect_special_characters() {
		assert!(!has_3_special_characters!(b"" 0 'a'), "Empty string");
		assert!(!has_3_special_characters!(b"a" 0 'a'), "Single character string");
		assert!(!has_3_special_characters!(b"aa" 0 'a'), "Double character string");
	}

	#[test]
	fn can_lex() {
		assert_eq!(block!(b"bbb\nb\nbbb" 0 0 'b'), (Block![b"b"], 9));
	}

	#[test]
	fn cannot_lex() {
		assert_eq!(block!(b"" 0 0 'c'), (Token::Empty, 0));
		assert_eq!(block!(b"c" 0 0 'c'), (Token::Invalid, 0));
		assert_eq!(block!(b"cc" 0 0 'c'), (Token::Invalid, 0));
	}
}
