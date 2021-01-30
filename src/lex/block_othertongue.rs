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

	macro_rules! BlockOthertongue {
		($($token:literal)*) => {
			create_block!(BlockOthertongue $($token)*)
		};
	}

	test_block_cases!{
		lexer: block_othertongue
		token creator: BlockOthertongue

		valid cases: [
			can_lex_empty_othertongue
			with sample b"===\n===" and tab count 0
			expecting [] with consumed size of 7 bytes.

			can_lex_othertongue_with_single_line
			with sample b"===\na\n===" and tab count 0
			expecting [b"a"] with consumed size of 9 bytes.

			can_lex_othertongue_with_indented_and_single_line
			with sample b"===\n\tbc\n\t===" and tab count 1
			expecting [b"\tbc"] with consumed size of 12 bytes.

			can_lex_othertongue_with_multiple_indented_lines
			with sample b"===\n\td\n\t\te\n\t\t===" and tab count 2
			expecting [b"\td" b"\t\te"] with consumed size of 16 bytes.

			can_lex_othertongue_with_empty_line
			with sample b"===\nf\n\n===" and tab count 0
			expecting [b"f" b""] with consumed size of 10 bytes.

			can_lex_othertongue_with_empty_lines
			with sample b"===\n\n\n\n\n\t===" and tab count 1
			expecting [b"" b"" b"" b""] with consumed size of 12 bytes.

			can_lex_othertongue_with_empty_line_and_indented_line
			with sample b"===\n\tg\n\nh\n\t===" and tab count 1
			 expecting [b"\tg" b"" b"h"] with consumed size of 14 bytes.
		]

		invalid cases: [
			cannot_lex_on_empty_line with sample b"" expecting Empty.
			cannot_lex_on_single_character_line with sample b"=" expecting Invalid.
			cannot_lex_on_double_character_line with sample b"==" expecting Invalid.
		]
	}
}
