/// Contains the valid tokens used for lexing and parsing.
#[cfg_attr(feature = "assertable_token", derive(Debug, PartialEq))]
pub enum Token<T, U> {
	ScopeLevel(usize),
	LineComment(T),
	BlockComment(U),
	Simplex(T),
	Complex(T),
	Attacher(T, T),
	LineOthertongue(T),
	BlockOthertongue(U)
}

mod simple_abstract_token;
mod dynamic_abstract_token;
