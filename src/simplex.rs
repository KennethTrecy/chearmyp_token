use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, TAB, VERTICAL_LINE};

enum SimplexEnding {
	None,
	Pad,
	End
}

pub fn simplex(src: &[u8], mut i: usize) -> TokenInfo {
	let limit = src.len();
	let mut size = 0;

	while i < limit {
		let ending = determine_ending(src, i, limit);
		if let SimplexEnding::None = ending {
			i += 1;
		} else {
			size = i;
			if let SimplexEnding::Pad = ending { i += 1; }
			break;
		}
	}

	(Token::Simplex(&src[0..size]), i)
}

fn determine_ending(src: &[u8], offset: usize, limit: usize)-> SimplexEnding {
	if src[offset] == VERTICAL_LINE {
		if offset + 1 == limit {
			SimplexEnding::End
		} else {
			let ending = src[offset + 1];
			if ending == NEW_LINE || ending == TAB {
				SimplexEnding::Pad
			} else {
				SimplexEnding::None
			}
		}
	} else {
		SimplexEnding::None
	}
}

#[cfg(test)]
mod tests {
	use super::{Token, simplex};

	#[test]
	fn can_lex() {
		macro_rules! test_simplex {
			(
				$sample:literal,
				$expected_token:expr,
				$expected_consumption:literal
			) => {
				let (token, consumed_size) = simplex($sample, 0);
				assert_eq!(token, $expected_token);
				assert_eq!(consumed_size, $expected_consumption);
			};
		}

		test_simplex!(b"a|	", Token::Simplex(&b"a"[..]), 2);
		test_simplex!(b"bc|	#", Token::Simplex(&b"bc"[..]), 3);
		test_simplex!(b"def|\n#", Token::Simplex(&b"def"[..]), 4);
	}
}
