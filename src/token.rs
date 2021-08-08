/// Contains the valid tokens used for lexing and parsing.
#[derive(Debug, PartialEq)]
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

mod abstract_token;
