use super::special_characters::NEW_LINE;

pub fn find_line_ending(src: &[u8], offset: usize, limit: usize)-> usize {
	let mut size = offset;

	while size < limit {
		if src[size] == NEW_LINE { break; }
		size += 1;
	}

	return size;
}
