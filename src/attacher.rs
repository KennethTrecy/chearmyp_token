use crate::token::{Token, TokenInfo};
use crate::special_characters::{COLON, NEW_LINE, TAB};
use crate::delimeter::Delimeter;

pub fn attacher(src: &[u8], slice_offset: usize, mut search_offset: usize) -> TokenInfo {
	let label_start = slice_offset;
	let label_end;

	loop {
		let separator = determine_separator(src, search_offset);
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
		match src.get(search_offset) {
			Some(&TAB) => search_offset += 1,
			Some(_) => break,
			None => return (Token::Invalid, search_offset)
		}
	}

	let content_start = search_offset;
	let content_end;

	loop {
		let ending = determine_ending(src, search_offset);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad | Delimeter::Limit => {
				content_end = search_offset;
				break;
			},
			Delimeter::Invalid => return (Token::Invalid, search_offset)
		}
	}

	let content = &src[content_start..content_end];
	(Token::Attacher(label, content), search_offset)
}

fn determine_separator(src: &[u8], offset: usize) -> Delimeter {
	match src.get(offset) {
		Some(&COLON) => {
			let next_offset = offset + 1;
			let next_character = src.get(next_offset);
			match next_character {
				Some(&TAB) => Delimeter::Pad,
				Some(_) => Delimeter::Incorrect,
				None => Delimeter::Invalid
			}
		},
		_ => Delimeter::Incorrect
	}
}

fn determine_ending(src: &[u8], offset: usize) -> Delimeter {
	match src.get(offset) {
		Some(&NEW_LINE) | Some(&TAB) => Delimeter::Pad,
		Some(_) => Delimeter::Incorrect,
		None => Delimeter::Limit
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

		test_attacher!(b"a:	b", Attacher!(b"a": b"b"), 4);
		test_attacher!(b"cd:		e", Attacher!(b"cd": b"e"), 6);
		test_attacher!(b"f:		g\n", Attacher!(b"f": b"g"), 5);
		test_attacher!(b"h:	i	j:	k", Attacher!(b"h": b"i"), 4);
	}
}
