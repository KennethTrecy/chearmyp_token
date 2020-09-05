use crate::find_line_ending;
use crate::token::{Token, TokenInfo};
use crate::special_characters::POUND_SIGN;

/// Returns the info of recognized line comment and its last index occupied in the source.
///
/// It needs an array of bytes as the first argument (known as source) and where to start looking
/// for the pound sign as the second argument (known as the offset). The token will not have a line
/// ending but it will be counted as the last index.
///
/// ## Notes
/// If the source has no pound sign found at the offset, it will return an empty token variant
/// with the offset.
///
/// ## Examples
/// ```
/// use chearmyp::line_comment::line_comment;
/// use chearmyp::token::Token;
///
/// let non_terminated = b"# hello world";
/// let (comment, last_index) = line_comment(&non_terminated[..], 0, non_terminated.len());
/// if let Token::LineComment(comment) = comment {
/// 	assert_eq!(comment, &non_terminated[1..]);
/// } else {
/// 	panic!("The returned token is not line comment.");
/// }
/// assert_eq!(last_index, 13);
///
/// let terminated = b"# hello world\n ";
/// let (comment, last_index) = line_comment(&terminated[..], 0, terminated.len());
/// if let Token::LineComment(comment) = comment {
/// 	assert_eq!(comment, &terminated[1..13]);
/// } else {
/// 	panic!("The returned token is not line comment.");
/// }
/// assert_eq!(last_index, 13);
///
/// let non_comment = b"hello world";
/// let (non_comment, last_index) = line_comment(&non_comment[..], 0, non_comment.len());
/// if let Token::Invalid = non_comment {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned token is not invalid.");
/// }
/// assert_eq!(last_index, 0);
/// ```
pub fn line_comment(src: &[u8], mut i: usize, limit: usize) -> TokenInfo {
	let first_character = src.get(i);
	if let Some(&POUND_SIGN) = first_character {
		i += 1;
		let end = find_line_ending(src, i, limit);
		(Token::LineComment(&src[i..end]), end)
	} else if let Some(_) = first_character {
		(Token::Invalid, i)
	} else {
		(Token::Empty, i)
	}
}

#[cfg(test)]
mod tests {
	use super::{Token, line_comment};

	#[test]
	fn can_lex() {
		macro_rules! test_line_comment {
			($sample:literal 0) => {
				let (token, line_comment_size) = line_comment($sample, 0, $sample.len());
				assert_eq!(line_comment_size, 0);
				assert_eq!(token, Token::Invalid);
			};
			($sample:literal $expected_size:literal $expected_token:expr) => {
				let (token, line_comment_size) = line_comment($sample, 0, $sample.len());
				assert_eq!(token, Token::LineComment(&$expected_token[..]),
					"Expected token of {:?}", $sample);
				assert_eq!(line_comment_size, $expected_size, "Expected length of {:?}", $sample);
			};
		}

		test_line_comment!(b"\n" 0);
		test_line_comment!(b"#\n" 1 b"");
		test_line_comment!(b"#" 1 b"");
		test_line_comment!(b"# hello" 7 b" hello");
		test_line_comment!(b"# hi\n" 4 b" hi");
	}

	#[test]
	fn cannot_lex() {
		assert_eq!(line_comment(&b""[..], 0, 0), (Token::Empty, 0));
	}
}
