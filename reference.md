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

## Simplex
Suffix: `|` then, new line or one or many tabs or end-of-file
These basic concepts cannot include other concepts, yet they can attach them.

## Complex
Suffix: new line or one to many tabs or end-of-file
These can contain other kinds of concepts by placing them to the next line with indention using a
tab.
