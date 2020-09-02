use crate::find_line_ending;
use crate::token::{Token, TokenInfo};
use crate::special_characters::POUND_SIGN;

pub fn line_comment(src: &[u8], mut i: usize) -> TokenInfo {
	if src[i] != POUND_SIGN { return TokenInfo(Token::Empty, i); }
	i += 1;

	let limit = src.len();
	let mut size = find_line_ending(src, i, limit);
	let end = if size == limit {
		limit
	} else {
		let new_line_index = size;
		size += 1;
		new_line_index
	};

	TokenInfo(Token::LineComment(&src[i..end]), size)
}

#[cfg(test)]
#[test]
fn can_lex() {
	macro_rules! test_line_comment {
		($sample:literal 0) => {
			let TokenInfo(token, line_comment_size) = line_comment($sample, 0);
			assert_eq!(line_comment_size, 0);
			assert_eq!(token, Token::Empty);
		};
		($sample:literal $expected_size:literal $expected_token:expr) => {
			let TokenInfo(token, line_comment_size) = line_comment($sample, 0);
			assert_eq!(token, Token::LineComment(&$expected_token[..]),
				"Expected token of {:?}", $sample);
			assert_eq!(line_comment_size, $expected_size, "Expected length of {:?}", $sample);
		};
	}
	test_line_comment!(b"\n" 0);
	test_line_comment!(b"#\n" 2 b"");
	test_line_comment!(b"#" 1 b"");
	test_line_comment!(b"# hello" 7 b" hello");
	test_line_comment!(b"# hi\n" 5 b" hi");
}
