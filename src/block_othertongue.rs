use crate::block::block;
use crate::token::{Token, TokenInfo};
use crate::special_characters::EQUAL;

pub fn block_othertongue(src: &[u8], i: usize, tab_count: usize) -> TokenInfo {
	let block = block(src, i, tab_count, EQUAL);
	if let (Token::Block(lines), last_seen_index) = block {
		(Token::BlockOthertongue(lines), last_seen_index)
	} else {
		block
	}
}
