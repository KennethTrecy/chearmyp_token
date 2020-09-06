use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, TAB, VERTICAL_LINE};
use crate::delimeter::Delimeter;

/// Returns the info of recognized simplex and the last index that has been checked from the source.
///
/// It needs an array of bytes as the first argument (known as source) and where to start looking
/// for the vertical line as the second argument (known as the offset).
///
/// ## Notes
/// It will return invalid token if there is no vertical line from the specified offset in source.
///
/// ## Examples
/// ```
/// use chearmyp::simplex;
/// use chearmyp::token::Token;
///
/// let terminated = b"hello world|";
/// let (token, last_index) = simplex(&terminated[..], 0);
/// if let Token::Simplex(token) = token {
/// 	assert_eq!(token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned token is not simplex.");
/// }
/// assert_eq!(last_index, 12);
///
/// let non_simplex = b"hello world";
/// let (non_simplex, last_index) = simplex(&non_simplex[..], 0);
/// if let Token::Invalid = non_simplex {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned token is not invalid.");
/// }
/// assert_eq!(last_index, 11);
/// ```
pub fn simplex(src: &[u8], mut offset: usize) -> TokenInfo {
	let start = offset;
	let end;

	loop {
		let ending = determine_ending(src, offset);
		match ending {
			Delimeter::Incorrect => offset += 1,
			Delimeter::Invalid => { return (Token::Invalid, offset); },
			Delimeter::Pad | Delimeter::Limit => {
				end = offset;
				offset += 1;
				break;
			}
		}
	}

	(Token::Simplex(&src[start..end]), offset)
}

fn determine_ending(src: &[u8], offset: usize) -> Delimeter {
	match src.get(offset) {
		Some(&VERTICAL_LINE) => {
			if let Some(&next_character) = src.get(offset + 1) {
				if next_character == NEW_LINE || next_character == TAB {
					Delimeter::Pad
				} else {
					Delimeter::Incorrect
				}
			} else {
				Delimeter::Limit
			}
		},
		Some(&NEW_LINE) | Some(&TAB) => Delimeter::Invalid,
		Some(_) => Delimeter::Incorrect,
		None => Delimeter::Invalid
	}
}

#[cfg(test)]
mod tests {
	use super::{Token, simplex};

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

	#[test]
	fn can_lex() {
		test_simplex!(b"a|	", Token::Simplex(&b"a"[..]), 2);
		test_simplex!(b"bc|	#", Token::Simplex(&b"bc"[..]), 3);
		test_simplex!(b"def|\n#", Token::Simplex(&b"def"[..]), 4);
		test_simplex!(b"kl|", Token::Simplex(&b"kl"[..]), 3);
	}

	#[test]
	fn cannot_lex_invalid_source() {
		test_simplex!(b"g\n", Token::Invalid, 1);
		test_simplex!(b"hi\tj", Token::Invalid, 2);
		test_simplex!(b"mn", Token::Invalid, 2);
	}
}
