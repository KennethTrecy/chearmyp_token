use crate::lex::token::{Token, TokenInfo};
use crate::lex::block::block;
use crate::lex::special_characters::POUND_SIGN;

/// Returns the info of recognized block comment and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the pound signs as the second argument (known as the offset), and the number of tabs must the
/// terminating pound signs be indented.
///
/// ## Notes
/// If the source has no 3 pound signs found at the offset, it will return an invalid token variant
/// with the offset.
///
/// ## Examples
/// ```
/// use chearmyp::lex::block_comment;
/// use chearmyp::lex::Token;
///
/// let terminated = b"###\n\thello world\n###";
/// let (comment, last_index) = block_comment(&terminated[..], 0, 0);
/// if let Token::BlockComment(comment) = comment {
/// 	assert_eq!(comment, vec![&b"\thello world"[..]]);
/// } else {
/// 	panic!("The returned token is not block comment.");
/// }
/// assert_eq!(last_index, 20);
///
/// let non_comment = b"hello world";
/// let (comment, last_index) = block_comment(&non_comment[..], 0, 0);
/// if let Token::Invalid = comment {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned token is not invalid.");
/// }
/// assert_eq!(last_index, 0);
/// ```
pub fn block_comment(src: &[u8], offset: usize, tab_count: usize) -> TokenInfo {
	let block = block(src, offset, tab_count, POUND_SIGN);
	if let (Token::Block(lines), last_seen_index) = block {
		(Token::BlockComment(lines), last_seen_index)
	} else {
		block
	}
}

#[cfg(test)]
mod t {
	use super::{Token, block_comment};

	macro_rules! test_block_comment {
		($sample:literal $expected_last_seen_index:literal $expected_token:expr) => {
			test_block_comment!{
				sample: $sample,
				tab_count: 0,
				consumed_size: $expected_last_seen_index,
				token: $expected_token
			}
		};
		(
			sample: $sample:expr,
			tab_count: $tab_count:literal,
			consumed_size: $expected_consumed_size:literal,
			token: $expected_token:expr
		) => {
			let (token, block_comment_size) = block_comment(
				$sample,
				0,
				$tab_count
			);
			assert_eq!(block_comment_size, $expected_consumed_size,
				"Consumed size of {:?}", $sample);
			assert_eq!(token, $expected_token,
				"Expected token of {:?}", $sample);
		};
	}

	macro_rules! BlockComment {
		($($token:literal)*) => {
			Token::BlockComment(alloc::vec![$(
				&$token[..],
			)*])
		};
	}

	#[test]
	fn can_lex_empty_comment() {
		test_block_comment!(b"###\n###" 7 BlockComment![]);
	}

	#[test]
	fn can_lex_comment_with_unindented_line() {
		test_block_comment!(
			sample: b"###\nhello world!\n###",
			tab_count: 0,
			consumed_size: 20,
			token: BlockComment![b"hello world!"]);
	}

	#[test]
	fn can_lex_comment_with_indented_line() {
		test_block_comment!(
			sample: b"###\n\thello world!\n\t###",
			tab_count: 1,
			consumed_size: 22,
			token: BlockComment![b"\thello world!"]);
	}

	#[test]
	fn can_lex_comment_with_indented_lines() {
		test_block_comment!(
			sample: "###\n\thello world!\n\t\thi universe\n\t\t###".as_bytes(),
			tab_count: 2,
			consumed_size: 37,
			token: BlockComment![b"\thello world!" b"\t\thi universe"]);
	}

	#[test]
	fn cannot_lex_empty_string() {
		assert_eq!(block_comment(&b""[..], 0, 0).0, Token::Empty);
	}

	#[test]
	fn cannot_lex_single_pound_sign() {
		assert_eq!(block_comment(&b"#"[..], 0, 0).0, Token::Invalid);
	}

	#[test]
	fn cannot_lex_double_pound_sign() {
		assert_eq!(block_comment(&b"##"[..], 0, 0).0, Token::Invalid);
	}
}
