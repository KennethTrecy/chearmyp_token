# Chearmyp
This is an experimental general purpose language. In this context, general purpose means that it can
be used as a markup, programming, command, and more.

## Syntax Overview
There are five general kinds of tokens that exist in Chearmyp. Take note that Chearmyp still
have unstable syntax.
1. *Comment*. These tokens may be used for documentation purposes. It has two subkinds: *line*
	comments and *block* comments.
	- *Line* comment. These are comments that only exist in one line and have a pound sign (`#`)
		prefix.
		```
		# This is a line comment.
		```
	- *Block* comment. These comments may have one or more lines. They have three pound signs (`###`)
	as their prefix and suffix.
			```
			###
			This is a block comment.
			###

			# Below is an indented block comment.
				###
					This is an indented block comment.
			### These pound signs will not terminate the comment.
					### This one too.
				Only the pound signs below will end the block comment.
				###
			```

2. *Simplex*. These tokens can be thought of as basic concepts that other concepts can use or
	include. For example, `letter`. Since these are simple, they cannot contain other concepts.
	Simplexes must end using a vertical line (`|`). It means that the concept "ends" there.
	```
	letter|	# This is an example of simplex. Use tabs to comment.
	1|	# A simplex can be a number.
	?|	# A simplex can be anything as long it does not start a pound sign.
	example city|	# And they may contain spaces too!
	```

3. *Complex*. These tokens are counterpart of *simplexes*. These are concepts that can contain other
	*simplexes* and *complexes*. For example `word`, it can contain the `letter|`. To express the
	containment of other tokens, those tokens must be indented using a tab.
	```
	# `word`, `punctuation`, and `binary` are examples of complex
	word
		letters
			letter|
			letter|
		part_of_speech|
		definition|

	punctuation
		.|
		?|
		!|

	binary
		0|
		1|
	```

4. *Attacher*. These tokens are pair of concepts attached to a *simplex* or *complex*. These
		concepts may be a metadata, adjective, reference, and others. They are written below or right
		side of the *simplex* or *complex* they are attaching to. *Attachers* must have a colon (`:`) and followed tab(s) or spaces(s) between the pairs namely label and content.
		```
		books
			book_a|
			price: $1					# Attaches book a's price.
			title: Title A		# Attaches book a's title.
			author: Author A	# Attaches book a's author.

			book_b|
			price:	$2				# Attaches book b's price.
			title:	Title B		# Attaches book b's title.
			author:	Author B	# Attaches book b's author.

			# Attachers can be written in one line.
			book_c|
			price: $3	title: Title C	author: Author C
		name: Book set A	# Attaches book set's name.
		```

5. *Othertongue*. These tokens contain a different language from the rest of the document. Their
	syntax is similar to *comments*. Due to the variety of languages, there are concepts that have no
	direct translation to the current concept. However, they must be contained to a *complex*.
	 - *Line* othertongue. These othertoungues are similar to *line* comments. Their containment must
		be expressed by indention using tab(s) or in one line after the *complex*. If a line
		othertongue has been expressed by indention, it must have an equal sign(`=`) and a space
		before it. If it has been inlined, they must have an equal sign(`=`) surrounded by spaces
		before it.
		```
		spelled_out_numbers
			= zero
			= one
			= two
			= ...

		# Kilig is a Filipino word with no direct translation to English
		excited butterflies = Kilig
		```
	- *Block* othertongue. These othertongues are similar to *block* comments. Instead of pound
		signs, they are identified using three equal signs(`===`). Their containment to a *complex*
		must be expressed by indention using tab(s).
		```
		languages
			natural
				English
					===
					Good morning!
					===
				Filipino
					===
					Magandang umaga!
					===
				Kapampangan
					===
					Mayap na abak!
					===
			programming
				===
				print "Good morning!"
				===
		```
