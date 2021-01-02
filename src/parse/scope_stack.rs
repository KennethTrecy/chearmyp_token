mod fragment;
mod relationship;

use alloc::vec::Vec;
use crate::parse::Node;
use fragment::Fragment;
use relationship::Relationship;

#[cfg(not(test))]
pub struct ScopeStack<'a> {
	level: usize,
	last_relationship: Relationship,
	fragments: Vec<Fragment<'a>>,
	scopes: Vec<Vec<Node<'a>>>
}

#[cfg(test)]
pub struct ScopeStack<'a> {
	pub level: usize,
	pub last_relationship: Relationship,
	pub fragments: Vec<Fragment<'a>>,
	pub scopes: Vec<Vec<Node<'a>>>
}

mod appenders;
mod push_to_last_scope;
mod promote_last_fragment;
mod push_to_preferred_relationship;

impl<'a> ScopeStack<'a> {
	/// Creates a scope stack that serves as the memory for the main parser.
	pub fn new() -> Self {
		let level = 0;
		let last_relationship = Relationship::Contained;
		let fragments = Vec::new();
		let mut scopes = Vec::with_capacity(1);

		scopes.push(Vec::new());

		Self {level, last_relationship, fragments, scopes}
	}
}
