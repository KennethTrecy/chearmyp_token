use super::special_characters::NEW_LINE;

/// Returns the index of the first line ending found in the source.
///
/// You can specify where to start looking for the line ending (known as offset). If there is no
/// line ending found from the offset up to the last index, the source's length will be returned.
///
/// ## Examples
/// ```
/// use chearmyp::find_line_ending;
///
/// let a = b"hello world";
/// assert_eq!(find_line_ending(&a[..], 0), 11, "Without line ending");
///
/// let a = b"hello\nworld\n";
/// assert_eq!(find_line_ending(&a[..], 0), 5, "Unskipped line ending");
/// assert_eq!(find_line_ending(&a[..], 6), 11, "Skipped line ending through offset");
/// ```
pub fn find_line_ending(src: &[u8], mut offset: usize)-> usize {
	loop {
		let character = src.get(offset);
		match character {
			Some(&NEW_LINE) | None => break,
			Some(_) => offset += 1
		}
	}

	return offset;
}
