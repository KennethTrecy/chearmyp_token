#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

mod abstracts {
	pub use abstract_chearmyp_source::{ AbstractSource, AbstractSourceCollection };
	pub use abstract_chearmyp_token::{
		AbstractToken,
		AbstractComplexToken,
		AbstractSimplexToken,
		AbstractAttacherToken,
		AbstractScopeLevelToken,
		AbstractLineCommentToken,
		AbstractBlockCommentToken,
		AbstractLineOthertongueToken,
		AbstractBlockOthertongueToken
	};
}

mod token_kind {
	pub use abstract_chearmyp_token::TokenKind;
}

/// Contains the data structures and type aliases used and/or returned by some lexers. They can be
/// used by both lexers and parsers.
mod token;

pub use token::Token;
