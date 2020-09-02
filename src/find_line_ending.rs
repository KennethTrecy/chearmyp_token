use super::special_characters::NEW_LINE;

/// Returns the index of the first line ending found in the source.
///
/// You can specify where to start looking for the line ending (known as offset) and stop when an
/// index has been reached (known as limit). If there is no line ending found within the limit, the
/// limit will returned.
///
/// ## Panics
/// It will panic if the limit is less than the source length.
///
/// ## Examples
/// ```
/// use chearmyp::find_line_ending;
///
/// let a = b"hello world";
/// assert_eq!(find_line_ending(&a[..], 0, a.len()), 11, "Without line ending");
///
/// let a = b"hello\nworld\n";
/// assert_eq!(find_line_ending(&a[..], 0, a.len()), 5, "Unskipped line ending");
/// assert_eq!(find_line_ending(&a[..], 6, a.len()), 11, "Skipped line ending through offset");
/// ```
pub fn find_line_ending(src: &[u8], offset: usize, limit: usize)-> usize {
	let mut size = offset;

	while size < limit {
		if src[size] == NEW_LINE { break; }
		size += 1;
	}

	return size;
}
