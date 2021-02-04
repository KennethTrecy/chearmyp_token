use crate::token::{Token, TokenInfo};
use crate::special_characters::{NEW_LINE, TAB, VERTICAL_LINE};
use crate::delimeter::Delimeter;

/// Returns the info of recognized simplex and the last index that has been checked from the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start slicing
/// (known as slice offset) as the second argument, and where to start looking for the vertical line
/// as the third argument (known as the search offset).
///
/// ## Notes
/// It will return invalid token if there is no vertical line from the specified offset in source.
/// Also, it does not differentiate attachers because there may be a case where the content of an
/// attacher ends in vertical line. Use [`attacher()`] lexer first.
///
/// ## Examples
/// ```
/// use chearmyp_lexer::simplex;
/// use chearmyp_lexer::Token;
///
/// let terminated = b"hello world|";
/// let (token, last_index) = simplex(&terminated[..], 0, 0);
/// if let Token::Simplex(token) = token {
/// 	assert_eq!(token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned token is not simplex.");
/// }
/// assert_eq!(last_index, 12);
///
/// let non_simplex = b"hello world";
/// let (non_simplex, last_index) = simplex(&non_simplex[..], 0, 0);
/// if let Token::Invalid = non_simplex {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned token is not invalid.");
/// }
/// assert_eq!(last_index, 11);
/// ```
///
/// [`attacher()`]: ./fn.attacher.html
pub fn simplex(src: &[u8], slice_offset: usize, mut search_offset: usize) -> TokenInfo {
	let start = slice_offset;
	let end;

	loop {
		let ending = determine_ending(src, search_offset);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Invalid => { return (Token::Invalid, search_offset); },
			Delimeter::Pad | Delimeter::Limit => {
				end = search_offset;
				search_offset += 1;
				break;
			}
		}
	}

	(Token::Simplex(&src[start..end]), search_offset)
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
mod t {
	use super::{Token, simplex};

	macro_rules! test_simplex {
		(
			$sample:literal,
			$expected_token:expr,
			$expected_consumption:literal
		) => {
			let (token, consumed_size) = simplex($sample, 0, 0);
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
	fn cannot_lex() {
		test_simplex!(b"g\n", Token::Invalid, 1);
		test_simplex!(b"hi\tj", Token::Invalid, 2);
		test_simplex!(b"mn", Token::Invalid, 2);
		test_simplex!(b"o: pq", Token::Invalid, 5);
	}
}
