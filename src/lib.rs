#![no_std]
const NEW_LINE: u8 = '\n' as u8;
const POUND_SIGN: u8 = '#' as u8;

#[cfg_attr(test, derive(Debug, PartialEq))]
enum Token<'a> {
	LineComment(&'a [u8]),
	Empty
}

pub struct TokenInfo<'a>(Token<'a>, usize);

pub fn line_comment(src: &[u8], mut i: usize) -> TokenInfo {
	if src[i] != POUND_SIGN { return TokenInfo(Token::Empty, i); }
	i += 1;

	let limit = src.len();
	let mut size = find_new_line(src, i, limit);
	let end = if size == limit {
		limit
	} else {
		let new_line_index = size;
		size += 1;
		new_line_index
	};

	TokenInfo(Token::LineComment(&src[i..end]), size)
}

fn find_new_line(src: &[u8], start: usize, limit: usize)-> usize {
	let mut size = start;

	while size < limit {
		if src[size] == NEW_LINE { break; }
		size += 1;
	}

	return size;
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
