use alloc::vec::Vec;
use crate::lex::find_line_ending;
use crate::lex::token::{Token, TokenInfo};
use crate::lex::special_characters::{NEW_LINE, TAB};

/// Returns the recognized block and the last seen index.
///
/// This is a generalization of blocks in cheamyp. It will return a vector of lines that are in the
/// block.
///
/// ## Example
/// ```
/// use chearmyp::lex::Token;
/// use chearmyp::lex::block;
///
/// let special_character = '@' as u8;
/// let sample_block = b"
/// @@@
/// hello world
/// @@@";
/// let (block, last_seen_index) = block(sample_block, 1, 0, special_character);
/// if let Token::Block(lines) = block {
/// 	assert_eq!(lines.len(), 1, "Expected lines of {:?}", &sample_block[..]);
///	assert_eq!(vec![&b"hello world"[..]], lines);
/// } else {
/// 	panic!("The source does not contain a block");
/// }
/// assert_eq!(last_seen_index, 20);
/// ```
pub fn block(src: &[u8], offset: usize, tab_count: usize, special_character: u8) -> TokenInfo {
	if has_3_special_characters(src, offset, special_character) {
		let mut lines = Vec::new();
		let mut offset = offset + 3;
		offset += if let Some(&NEW_LINE) = src.get(offset) { 1 } else { 0 };

		loop {
			let start = offset;
			let end = find_line_ending(src, start);
			if start == end && src.get(end) == None { break; }
			let line = &src[start..end];

			let mut indent_size = tab_count;
			while indent_size > 0 {
				indent_size -= 1;
				if line.get(indent_size) != Some(&TAB) { break; }
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
	fn cannot_detect_special_characters_on_empty_line() {
		assert!(!has_3_special_characters!(b"" 0 'a'), "Empty string");
	}

	#[test]
	fn cannot_detect_special_characters_on_single_character_line() {
		assert!(!has_3_special_characters!(b"a" 0 'a'), "Single-character string");
	}

	#[test]
	fn cannot_detect_special_characters_on_double_character_line() {
		assert!(!has_3_special_characters!(b"aa" 0 'a'), "Double-character string");
	}

	#[test]
	fn can_lex_with_proper_content() {
		assert_eq!(block!(b"bbb\nb\nbbb" 0 0 'b'), (Block![b"b"], 9));
	}

	#[test]
	fn can_lex_with_an_empty_line() {
		assert_eq!(block!(b"ddd\nd\n\ndd\nddd" 0 0 'd'), (Block![b"d" b"" b"dd"], 13));
	}

	#[test]
	fn can_lex_with_empty_lines() {
		assert_eq!(block!(b"eee\n\n\neee\n" 0 0 'e'), (Block![b"" b""], 10));
	}

	#[test]
	fn can_lex_with_empty_line_and_tabbed_line() {
		assert_eq!(block!(b"fff\nf\n\tf\n\nf\n\tfff" 0 1 'f'), (Block![b"f" b"\tf" b"" b"f"], 16));
	}

	#[test]
	fn cannot_lex_on_empty_line() {
		assert_eq!(block!(b"" 0 0 'c'), (Token::Empty, 0));
	}

	#[test]
	fn cannot_lex_on_single_character_line() {
		assert_eq!(block!(b"c" 0 0 'c'), (Token::Invalid, 0));
	}

	#[test]
	fn cannot_lex_on_double_character_line() {
		assert_eq!(block!(b"cc" 0 0 'c'), (Token::Invalid, 0));
	}
}
