use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, SPACE, TAB};
use crate::delimeter::Delimeter;
use crate::line_othertongue::determine_othertongue_prefix;

pub fn complex(src: &[u8], slice_offset: usize, mut search_offset: usize) -> TokenInfo {
	let limit = src.len();
	let size;

	loop {
		let ending = determine_ending(src, search_offset, limit);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad => {
				size = search_offset;
				break;
			},
			Delimeter::Limit => {
				size = search_offset + 1;
				break;
			},
			Delimeter::Invalid => return (Token::Invalid, search_offset)
		}
	}

	(Token::Complex(&src[slice_offset..size]), search_offset)
}

pub(crate) fn determine_ending(src: &[u8], offset: usize, limit: usize) -> Delimeter {
	match src[offset] {
		NEW_LINE | TAB => Delimeter::Pad,
		SPACE => {
			if let Delimeter::Pad = determine_othertongue_prefix(src, offset) {
				Delimeter::Pad
			} else {
				Delimeter::Incorrect
			}
		},
		_ => {
			if offset + 1 == limit {
				Delimeter::Limit
			} else {
				Delimeter::Incorrect
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Token, complex};

	#[test]
	fn can_lex() {
		macro_rules! test_complex {
			(
				$sample:literal,
				$expected_token:expr,
				$expected_consumption:literal
			) => {
				let (token, consumed_size) = complex($sample, 0, 0);
				assert_eq!(token, $expected_token);
				assert_eq!(consumed_size, $expected_consumption);
			};
		}

		test_complex!(b"a", Token::Complex(&b"a"[..]), 0);
		test_complex!(b"bc	", Token::Complex(&b"bc"[..]), 2);
		test_complex!(b"d\n", Token::Complex(&b"d"[..]), 1);
		test_complex!(b"e = f\n", Token::Complex(&b"e"[..]), 1);
	}
}
