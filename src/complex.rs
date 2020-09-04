use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, TAB};

enum ComplexEnding {
	None,
	Pad,
	Limit
}

pub fn complex(src: &[u8], slice_offset: usize, mut search_offset: usize) -> TokenInfo {
	let limit = src.len();
	let size;

	loop {
		let ending = determine_ending(src, search_offset, limit);
		match ending {
			ComplexEnding::None => search_offset += 1,
			ComplexEnding::Pad => {
				size = search_offset;
				break;
			},
			ComplexEnding::Limit => {
				size = search_offset + 1;
				break;
			}
		}
	}

	(Token::Complex(&src[slice_offset..size]), search_offset)
}

fn determine_ending(src: &[u8], offset: usize, limit: usize)-> ComplexEnding {
	match src[offset] {
		NEW_LINE | TAB => ComplexEnding::Pad,
		_ => {
			if offset + 1 == limit {
				ComplexEnding::Limit
			} else {
				ComplexEnding::None
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
	}
}
