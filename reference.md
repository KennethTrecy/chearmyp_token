# Chearmyp Syntax Reference
These document contains specific details about the tokens' syntax in Chearmyp.

## Comment
In general, comments must be denoted with a pound sign (`#`).

### Line comment
Prefix: `#`
These comments must within one line only.

### Block comment
Prefix: zero to many tabs then, `###` then new line
Suffix: zero to many tabs (must match number of tabs in prefix) then, `###` then, new line or
				end-of-file.
These comments can have multiple lines. There contents must be preserved excluding the prefix and
suffix.
