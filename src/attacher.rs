use crate::token::{Token, TokenInfo};
use crate::special_characters::{COLON, NEW_LINE, TAB};
use crate::delimeter::Delimeter;
use crate::complex::determine_ending;

pub fn attacher(src: &[u8], slice_offset: usize, mut search_offset: usize) -> TokenInfo {
	let limit = src.len();
	let label_start = slice_offset;
	let label_end;

	loop {
		let separator = determine_separator(src, search_offset, limit);
		match separator {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad => {
				label_end = search_offset;
				search_offset += 1;
				break;
			},
			_ => return (Token::Invalid, search_offset)
		}
	}

	let label = &src[label_start..label_end];

	loop {
		match src[search_offset] {
			TAB => search_offset += 1,
			_ => break
		}
		if search_offset == limit { return (Token::Invalid, search_offset); }
	}

	let content_start = search_offset;
	let content_end;

	loop {
		let ending = determine_ending(src, search_offset, limit);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad => {
				content_end = search_offset;
				break;
			},
			Delimeter::Limit => {
				content_end = search_offset + 1;
				break;
			},
			Delimeter::Invalid => return (Token::Invalid, search_offset)
		}
	}

	let content = &src[content_start..content_end];
	(Token::Attacher(label, content), search_offset)
}

fn determine_separator(src: &[u8], offset: usize, limit: usize) -> Delimeter {
	match src[offset] {
		COLON => {
			let next_offset = offset + 1;
			let has_reached_limit = next_offset == limit;
			let next_character = src[next_offset];
			if !has_reached_limit && next_character == TAB {
				Delimeter::Pad
			} else if has_reached_limit || next_character == NEW_LINE {
				Delimeter::Invalid
			} else {
				Delimeter::Incorrect
			}
		},
		_ => Delimeter::Incorrect
	}
}
