use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{COLON, NEW_LINE, SPACE, TAB};
use crate::delimeter::Delimeter;

/// Returns the info of recognized attacher and the last index that has been checked from the
/// source.
///
/// It needs an array of bytes as the first argument (known as source), where to start slicing
/// (known as slice offset) as the second argument, and where to start looking for the terminator
/// (such as tab, new line, or equal sign of the inlined othertongue) as the third argument (known
/// as the search offset).
///
/// ## Notes
/// If there is no valid raw_token found, it will return invalid raw_token along with the last index
/// checked.
///
/// ## Examples
/// ```
/// use chearmyp_lexer::attacher;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"hello:	world";
/// let (raw_token, last_index) = attacher(&non_terminated[..], 0, 0);
/// if let RawToken::Attacher(label, content) = raw_token {
/// 	assert_eq!(label, &b"hello"[..]);
/// 	assert_eq!(content, &b"world"[..]);
/// } else {
/// 	panic!("The returned raw_token is not attacher.");
/// }
/// assert_eq!(last_index, 12);
///
/// let terminated = b"hello:	world\n";
/// let (raw_token, last_index) = attacher(&terminated[..], 0, 0);
/// if let RawToken::Attacher(label, content) = raw_token {
/// 	assert_eq!(label, &b"hello"[..]);
/// 	assert_eq!(content, &b"world"[..]);
/// } else {
/// 	panic!("The returned raw_token is not attacher.");
/// }
/// assert_eq!(last_index, 12);
///
/// let simplex = b"hello world";
/// let (raw_token, last_index) = attacher(&simplex[..], 0, 0);
/// if let RawToken::Invalid = raw_token {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned raw_token is not invalid");
/// }
/// assert_eq!(last_index, 11);
/// ```
pub fn attacher(src: &[u8], slice_offset: usize, mut search_offset: usize) -> RawTokenInfo {
	let label_start = slice_offset;
	let label_end;

	loop {
		let separator = determine_separator(src, search_offset);
		match separator {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad => {
				label_end = search_offset;
				search_offset += 1;
				break;
			},
			_ => return (RawToken::Invalid, search_offset)
		}
	}

	let label = &src[label_start..label_end];

	loop {
		match src.get(search_offset) {
			Some(&TAB) | Some(&SPACE) => search_offset += 1,
			Some(_) => break,
			None => return (RawToken::Invalid, search_offset)
		}
	}

	let content_start = search_offset;
	let content_end;

	loop {
		let ending = determine_ending(src, search_offset);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad | Delimeter::Limit => {
				content_end = search_offset;
				break;
			},
			Delimeter::Invalid => return (RawToken::Invalid, search_offset)
		}
	}

	let content = &src[content_start..content_end];
	(RawToken::Attacher(label, content), search_offset)
}

fn determine_separator(src: &[u8], offset: usize) -> Delimeter {
	match src.get(offset) {
		Some(&COLON) => {
			let next_offset = offset + 1;
			let next_character = src.get(next_offset);
			match next_character {
				Some(&TAB) | Some(&SPACE) => Delimeter::Pad,
				Some(&NEW_LINE) | None => Delimeter::Invalid,
				Some(_) => Delimeter::Incorrect
			}
		},
		Some(&NEW_LINE) | Some(&TAB) => Delimeter::Invalid,
		Some(_) => Delimeter::Incorrect,
		None => Delimeter::Limit
	}
}

fn determine_ending(src: &[u8], offset: usize) -> Delimeter {
	match src.get(offset) {
		Some(&NEW_LINE) | Some(&TAB) => Delimeter::Pad,
		Some(_) => Delimeter::Incorrect,
		None => Delimeter::Limit
	}
}

#[cfg(test)]
mod t {
	use super::{RawToken, attacher};

	macro_rules! test_attacher {
		(
			$sample:literal,
			$expected_token:expr,
			$expected_consumption:literal
		) => {
			let (raw_token, consumed_size) = attacher($sample, 0, 0);
			assert_eq!(raw_token, $expected_token);
			assert_eq!(consumed_size, $expected_consumption);
		};
	}

	macro_rules! Attacher {
		($label:literal : $content:literal) => {
			RawToken::Attacher(&$label[..], &$content[..])
		};
	}

	#[test]
	fn can_lex() {
		test_attacher!(b"a:	b", Attacher!(b"a": b"b"), 4);
		test_attacher!(b"cd:		e", Attacher!(b"cd": b"e"), 6);
		test_attacher!(b"f:		g\n", Attacher!(b"f": b"g"), 5);
		test_attacher!(b"h:	i	j:	k", Attacher!(b"h": b"i"), 4);
	}

	#[test]
	fn cannot_lex() {
		test_attacher!(b"lm", RawToken::Invalid, 2);
		test_attacher!(b"n|", RawToken::Invalid, 2);
		test_attacher!(b"o:	", RawToken::Invalid, 3);
	}

	#[test]
	fn can_lex_separated_by_colon_then_space() {
		test_attacher!(b"p: q", Attacher!(b"p": b"q"), 4);
	}
}
