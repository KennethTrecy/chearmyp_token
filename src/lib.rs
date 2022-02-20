#![cfg_attr(feature = "no_std", no_std)]

//! # Chearmyp Token
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.
//! - `assertable_token`. Allows token to be used in tests.

#[cfg(feature = "no_std")]
#[cfg_attr(test, macro_use)]
extern crate alloc;

mod native {
	#[cfg(feature = "no_std")]
	pub use core::{
		ops::Range,
		marker::PhantomData
	};

	#[cfg(feature = "no_std")]
	pub use alloc::vec::Vec;

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		vec::Vec,
		ops::Range,
		marker::PhantomData
	};
}

mod abstracts {
	pub use abstract_chearmyp_boundary::{AbstractBoundary, AbstractBoundaryCollection};
	pub use abstract_chearmyp_token::{
		SimpleAbstractToken,
		DynamicAbstractToken,
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
