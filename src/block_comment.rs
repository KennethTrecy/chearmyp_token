use crate::raw_token::{RawToken, RawTokenInfo};
use crate::block::block;
use crate::special_characters::POUND_SIGN;

/// Returns the info of recognized block comment and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the pound signs as the second argument (known as the offset), and the number of tabs must the
/// terminating pound signs be indented.
///
/// ## Notes
/// If the source has no 3 pound signs found at the offset, it will return an invalid raw_token variant
/// with the offset.
///
/// ## Examples
/// ```
/// use chearmyp_lexer::block_comment;
/// use chearmyp_lexer::RawToken;
///
/// let terminated = b"###\n\thello world\n###";
/// let (comment, last_index) = block_comment(&terminated[..], 0, 0);
/// if let RawToken::BlockComment(comment) = comment {
/// 	assert_eq!(comment, vec![&b"\thello world"[..]]);
/// } else {
/// 	panic!("The returned raw_token is not block comment.");
/// }
/// assert_eq!(last_index, 20);
///
/// let non_comment = b"hello world";
/// let (comment, last_index) = block_comment(&non_comment[..], 0, 0);
/// if let RawToken::Invalid = comment {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned raw_token is not invalid.");
/// }
/// assert_eq!(last_index, 0);
/// ```
pub fn block_comment(src: &[u8], offset: usize, tab_count: usize) -> RawTokenInfo {
	let block = block(src, offset, tab_count, POUND_SIGN);
	if let (RawToken::Block(lines), last_seen_index) = block {
		(RawToken::BlockComment(lines), last_seen_index)
	} else {
		block
	}
}

#[cfg(test)]
mod t {
	use super::{RawToken, block_comment};

	macro_rules! BlockComment {
		($($raw_token:literal)*) => {
			create_block!(BlockComment $($raw_token)*)
		};
	}

	test_block_cases!{
		lexer: block_comment
		raw_token creator: BlockComment

		valid cases: [
			can_lex_empty_comment with sample b"###\n###" and tab count 0
			expecting [] with consumed size of 7 bytes.

			can_lex_comment_with_unindented_line
			with sample b"###\nhello world!\n###" and tab count 0
			expecting [b"hello world!"] with consumed size of 20 bytes.

			can_lex_comment_with_indented_line
			with sample b"###\n\thello world!\n\t###" and tab count 1
			expecting [b"\thello world!"] with consumed size of 22 bytes.

			can_lex_comment_with_indented_lines
			with sample b"###\n\thello world!\n\t\thi universe\n\t\t###" and tab count 2
			expecting [b"\thello world!" b"\t\thi universe"] with consumed size of 37 bytes.

			can_lex_comment_with_empty_line
			with sample b"###\n\n\thello world\n\t###" and tab count 1
			expecting [b"" b"\thello world"] with consumed size of 22 bytes.

			can_lex_comment_with_empty_lines
			with sample b"###\n\n\n\n###" and tab count 0
			expecting [b"" b"" b""] with consumed size of 10 bytes.

			can_lex_comment_with_empty_line_and_indented_line
			with sample b"###\n\t\thello world!\n\nhi universe\n\t###" and tab count 1
			expecting [b"\t\thello world!" b"" b"hi universe"] with consumed size of 36 bytes.
		]

		invalid cases: [
			cannot_lex_empty_string with sample b"" expecting Empty.
			cannot_lex_single_pound_sign with sample b"#" expecting Invalid.
			cannot_lex_double_pound_sign with sample b"##" expecting Invalid.
		]
	}
}
