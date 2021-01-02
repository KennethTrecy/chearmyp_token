use alloc::vec::Vec;
use crate::parse::Node;

/// Contains the fragments used for parsing.
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Fragment<'a> {
	Simplex(&'a [u8], Vec<Node<'a>>),
	Complex(&'a [u8], Vec<Node<'a>>)
}
