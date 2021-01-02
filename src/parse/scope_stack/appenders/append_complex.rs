use alloc::vec::Vec;
use crate::parse::scope_stack::Fragment;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends a complex fragment to the collection of fragments.
	pub fn append_complex(&mut self, concept: &'a [u8]) {
		let complex_fragment = Fragment::Complex(concept, Vec::new());
		self.fragments.push(complex_fragment);
		self.scopes.push(Vec::new());
	}
}

#[cfg(test)]
mod t {
	use crate::parse::scope_stack::Relationship;
	use super::{Fragment, ScopeStack, Vec};

	#[test]
	pub fn can_append() {
		let concept = b"ab";
		let mut scope_stack = ScopeStack::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::Complex(&concept[..], Vec::new());
			fragments.push(fragment);
			fragments
		};

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let scope = Vec::new();
			scopes.push(scope);

			let complex_scope = Vec::new();
			scopes.push(complex_scope);

			scopes
		};

		scope_stack.append_complex(&concept[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
