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

#[cfg(test)]
mod tests {
	use super::{Token, attacher};

	#[test]
	fn can_lex_attacher() {
		macro_rules! test_attacher {
			(
				$sample:literal,
				$expected_token:expr,
				$expected_consumption:literal
			) => {
				let (token, consumed_size) = attacher($sample, 0, 0);
				assert_eq!(token, $expected_token);
				assert_eq!(consumed_size, $expected_consumption);
			};
		}

		macro_rules! Attacher {
			($label:literal : $content:literal) => {
				Token::Attacher(&$label[..], &$content[..])
			};
		}

		test_attacher!(b"a:	b", Attacher!(b"a": b"b"), 3);
		test_attacher!(b"cd:		e", Attacher!(b"cd": b"e"), 5);
		test_attacher!(b"f:		g\n", Attacher!(b"f": b"g"), 5);
		test_attacher!(b"h:	i	j:	k", Attacher!(b"h": b"i"), 4);
	}
}
