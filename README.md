# Chearmyp
This is an experimental general purpose language. In this context, general purpose means that it can
be used as a markup, programming, command, and more.

## Syntax
There are five general kinds of tokens that exist in Churmyprea. Take note that Churmyprea still
have unstable syntax.
1. *Comments*. These tokens is used for documentation purposes. It has two subkinds: *line* comments
	and *block* comments.
    - *Line* comment. These are comments that only exist in one line. They are denoted using a pound
      sign (`#`).
		```
		# This is a line comment.
		```
    - *Block* comment. These comments may have one or more lines. It must have three pound signs
      (`###`) before and after the comment content. If the starting three pound signs (`###`) have
      indention before them, the ending three pound signs (`###`) must also have the matching
      indention.
		```
		###
		This is a block comment
		###

		# Below is an indented block comment
			###
				This is an indented block comment
		### These pound signs will not terminate the comment
				### This one too
			Only the pound signs below will end the block comment.
			###
		```
