macro_rules! create_block {
	($variant:ident $($token:literal)*) => {
		Token::$variant(alloc::vec![$(&$token[..],)*])
	};
}
