use crate::block::block;
use crate::token::{Token, TokenInfo};
use crate::special_characters::POUND_SIGN;

/// Returns the info of recognized block comment and its last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the pound signs as the second argument (known as the offset), and the number of tabs must the
/// terminatng pound signs be indented.
///
/// ## Notes
/// If the source has no 3 pound signs found at the offset, it will return an invalid token variant
/// with the offset.
///
/// ## Panics
/// It cannot lex a source that is less than 3 characters or if not terminated properly.
///
/// ## Examples
/// ```
/// use chearmyp::block_comment::block_comment;
/// use chearmyp::token::Token;
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
pub fn block_comment(src: &[u8], i: usize, tab_count: usize) -> TokenInfo {
	let block = block(src, i, tab_count, POUND_SIGN);
	if let (Token::Block(lines), last_seen_index) = block {
		(Token::BlockComment(lines), last_seen_index)
	} else {
		block
	}
}

#[cfg(test)]
mod tests {
	use alloc::vec::Vec;
	use super::{Token, block_comment};

	#[test]
	fn can_lex() {
		macro_rules! test_block_comment {
			(
				$sample:literal
				$expected_consumed_size:literal
				$expected_token:ident
			) => {
				test_block_comment!{
					sample: $sample,
					tab_count: 0,
					consumed_size: $expected_consumed_size,
					token: $expected_token
				}
			};
			(
				sample: $sample:expr,
				tab_count: $tab_count:literal,
				consumed_size: $expected_consumed_size:literal,
				token: $expected_token:ident
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

		let expected_token = Token::BlockComment(Vec::new());
		test_block_comment!(b"###\n###" 7 expected_token);

		let expected_token = Token::BlockComment(alloc::vec![&b"hello world!"[..]]);
		test_block_comment!(
			sample: b"###\nhello world!\n###",
			tab_count: 0,
			consumed_size: 20,
			token: expected_token);

		let expected_token = Token::BlockComment(alloc::vec![&b"\thello world!"[..]]);
		test_block_comment!(
			sample: b"###\n\thello world!\n\t###",
			tab_count: 1,
			consumed_size: 22,
			token: expected_token);

		let expected_token = Token::BlockComment(alloc::vec![
			&b"\thello world!"[..],
			&b"\t\thi universe"[..]]);
		test_block_comment!(
			sample: "###\n\thello world!\n\t\thi universe\n\t\t###".as_bytes(),
			tab_count: 2,
			consumed_size: 37,
			token: expected_token);
	}

	#[test]
	#[should_panic]
	fn cannot_lex() {
		block_comment(&b""[..], 0, 0);
	}
}
