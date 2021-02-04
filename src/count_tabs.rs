use crate::special_characters::TAB;

/// Returns the number of initial tabs in the source.
///
/// It needs an array of bytes as the first argument (known as source), and the previous number of
/// tabs worked on (known as old tab count). If it is the first time to check the number of initial
/// tabs, set the old tab count to 0.
pub fn count_tabs(src: &[u8], old_tab_count: usize) -> usize {
	let mut new_tab_count = old_tab_count;

	loop {
		match src.get(new_tab_count) {
			Some(&TAB) => new_tab_count += 1,
			Some(_) => {
				if new_tab_count > 0 {
					match src[new_tab_count - 1] {
						TAB => break,
						_ => new_tab_count -= 1
					}
				} else {
					break;
				}
			},
			None => {
				if new_tab_count > 0 {
					new_tab_count -= 1
				} else {
					break;
				}
			}
		}
	}

	new_tab_count
}

#[cfg(test)]
mod t {
	use super::count_tabs;

	#[test]
	fn can_count_on_first_time() {
		let sample = b"a";
		let old_tab_count = 0;
		let expected_new_tab_count = 0;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_count_decreased_tabs() {
		let sample = b"bcd";
		let old_tab_count = 3;
		let expected_new_tab_count = 0;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_count_remain_tab_count() {
		let sample = b"\te";
		let old_tab_count = 1;
		let expected_new_tab_count = 1;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_count_increased_tabs() {
		let sample = b"\t\tfg";
		let old_tab_count = 1;
		let expected_new_tab_count = 2;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}
}
