use crate::lex::block::block;
use crate::lex::token::{Token, TokenInfo};
use crate::lex::special_characters::EQUAL;

/// Returns the info of recognized block othertongue and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the equal signs as the second argument (known as the offset), and the number of tabs must the
/// terminating equal signs be indented.
///
/// ## Notes
/// If the source has no 3 equal signs found at the offset, it will return an invalid token variant
/// with the offset.
///
/// ## Examples
/// ```
/// use chearmyp::lex::block_othertongue;
/// use chearmyp::lex::Token;
///
/// let terminated = b"===\n\thello world\n===\n";
/// let (block, last_index) = block_othertongue(&terminated[..], 0, 0);
/// if let Token::BlockOthertongue(othertongue) = block {
/// 	assert_eq!(othertongue, vec![&b"\thello world"[..]]);
/// } else {
/// 	panic!("The returned token is not block othertongue.");
/// }
/// assert_eq!(last_index, 21);
///
/// let non_othertongue = b"hello world";
/// let (token, last_index) = block_othertongue(&non_othertongue[..], 0, 0);
/// if let Token::Invalid = token {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned token is not invalid.");
/// }
/// assert_eq!(last_index, 0);
/// ```
pub fn block_othertongue(src: &[u8], offset: usize, tab_count: usize) -> TokenInfo {
	let block = block(src, offset, tab_count, EQUAL);
	if let (Token::Block(lines), last_seen_index) = block {
		(Token::BlockOthertongue(lines), last_seen_index)
	} else {
		block
	}
}

#[cfg(test)]
mod t {
	use super::{Token, block_othertongue};

	macro_rules! test_block_othertongue {
		($sample:literal $tab_count:literal $expected_consumed_size:literal $expected_token:expr) => {
			let (token, block_othertongue_size) = block_othertongue($sample, 0, $tab_count);
			assert_eq!(block_othertongue_size, $expected_consumed_size,
				"Consumed size of {:?}", $sample);
			assert_eq!(token, $expected_token,
				"Expected token of {:?}", $sample);
		};
	}

	macro_rules! BlockOthertongue {
		($($token:literal)*) => {
			create_block!(BlockOthertongue $($token)*)
		};
	}

	#[test]
	fn can_lex_empty_othertongue() {
		test_block_othertongue!(b"===\n===" 0 7 BlockOthertongue![]);
	}

	#[test]
	fn can_lex_othertongue_with_single_line() {
		test_block_othertongue!(b"===\na\n===" 0 9 BlockOthertongue![b"a"]);
	}

	#[test]
	fn can_lex_othertongue_with_indented_and_single_line() {
		test_block_othertongue!(b"===\n\tbc\n\t===" 1 12 BlockOthertongue![b"\tbc"]);
	}

	#[test]
	fn can_lex_othertongue_with_multiple_indented_lines() {
		test_block_othertongue!(b"===\n\td\n\t\te\n\t\t===" 2 16 BlockOthertongue![b"\td" b"\t\te"]);
	}

	#[test]
	fn can_lex_othertongue_with_empty_line() {
		test_block_othertongue!(b"===\nf\n\n===" 0 10 BlockOthertongue![b"f" b""]);
	}

	#[test]
	fn can_lex_othertongue_with_empty_lines() {
		test_block_othertongue!(b"===\n\n\n\n\n\t===" 1 12 BlockOthertongue![b"" b"" b"" b""]);
	}

	#[test]
	fn can_lex_othertongue_with_empty_line_and_indented_line() {
		test_block_othertongue!(b"===\n\tg\n\nh\n\t===" 1 14 BlockOthertongue![b"\tg" b"" b"h"]);
	}

	#[test]
	fn cannot_lex_on_empty_line() {
		assert_eq!(block_othertongue(&b""[..], 0, 0).0, Token::Empty);
	}

	#[test]
	fn cannot_lex_on_single_character_line() {
		assert_eq!(block_othertongue(&b"="[..], 0, 0).0, Token::Invalid);
	}

	#[test]
	fn cannot_lex_on_double_character_line() {
			assert_eq!(block_othertongue(&b"=="[..], 0, 0).0, Token::Invalid);
	}
}
