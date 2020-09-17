use crate::block::block;
use crate::token::{Token, TokenInfo};
use crate::special_characters::EQUAL;

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
/// use chearmyp::block_othertongue;
/// use chearmyp::token::Token;
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
			Token::BlockOthertongue(alloc::vec![$(
				&$token[..],
			)*])
		};
	}

	#[test]
	fn can_lex() {
		test_block_othertongue!(b"===\n===" 0 7 BlockOthertongue![]);
		test_block_othertongue!(b"===\na\n===" 0 9 BlockOthertongue![b"a"]);
		test_block_othertongue!(b"===\n\tbc\n\t===" 1 12 BlockOthertongue![b"\tbc"]);
		test_block_othertongue!(b"===\n\td\n\t\te\n\t\t===" 2 16 BlockOthertongue![b"\td" b"\t\te"]);
	}

	#[test]
	fn cannot_lex() {
		assert_eq!(block_othertongue(&b""[..], 0, 0).0, Token::Empty);
		assert_eq!(block_othertongue(&b"="[..], 0, 0).0, Token::Invalid);
		assert_eq!(block_othertongue(&b"=="[..], 0, 0).0, Token::Invalid);
	}
}
