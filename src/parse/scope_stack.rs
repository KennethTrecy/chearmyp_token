mod fragment;

use alloc::vec::Vec;
use crate::parse::Node;

#[cfg(not(test))]
pub struct ScopeStack<'a> {
	level: usize,
	scopes: Vec<Vec<Node<'a>>>
}

#[cfg(test)]
pub struct ScopeStack<'a> {
	pub level: usize,
	pub scopes: Vec<Vec<Node<'a>>>
}

impl<'a> ScopeStack<'a> {
	/// Creates a scope stack that serves as the memory for the main parser.
	pub fn new() -> Self {
		let level = 0;
		let mut scopes = Vec::with_capacity(1);

		scopes.push(Vec::new());

		Self {level, scopes}
	}
}
