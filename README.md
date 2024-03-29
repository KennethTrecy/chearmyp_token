[![Library Tests](https://img.shields.io/github/actions/workflow/status/KennethTrecy/chearmyp_token/library.yml?style=for-the-badge)](https://github.com/KennethTrecy/chearmyp_token/actions/workflows/library.yml)
![GitHub lines](https://img.shields.io/github/license/KennethTrecy/chearmyp_token?style=for-the-badge)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/KennethTrecy/chearmyp_token?style=for-the-badge&display_name=tag&sort=semver)
![GitHub closed issues count](https://img.shields.io/github/issues-closed/KennethTrecy/chearmyp_token?style=for-the-badge)
![GitHub pull request count](https://img.shields.io/github/issues-pr-closed/KennethTrecy/chearmyp_token?style=for-the-badge)
![Commits since latest version](https://img.shields.io/github/commits-since/KennethTrecy/chearmyp_token/latest?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/KennethTrecy/chearmyp_token?style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/repo-size/KennethTrecy/chearmyp_token?style=for-the-badge)

# Chearmyp Token
A concrete implementation of tokens for Chearmyp language.

## Installation
Add it to the dependencies:
```
[dependencies.chearmyp_token]
git = "https://github.com/KennethTrecy/chearmyp_token"
tag = "v1.0.0"
```

You may also activate all the features:
```
[dependencies.chearmyp_token]
git = "https://github.com/KennethTrecy/chearmyp_token"
tag = "v1.0.0"
features = ["no_std", "assertable_token"]
```

## Origin
It was in a repository with the [lexer library] of Chearmyp. Yet, it has been forked for better
development of future libraries.

Some parts of the repository were based from [`filled_bare_metal`] branch of [Feo Template].

## Usage

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

### Initialization
If you want to contribute, this repository should be initialized to adhere in [Conventional Commits specification] for organize
commits and automated generation of change log.

#### Prerequisites
- [Node.js and NPM]
- [pnpm] (optional)

#### Instructions
By running the command below, all your commits will be linted to follow the [Conventional Commits
specification].
```
$ npm install
```

Or if you have installed [pnpm], run the following command:
```
$ pnpm install
```

To generate the change log automatically, run the command below:
```
$ npx changelogen --from=[tag name or branch name or commit itself] --to=master
```

## Tokens
The tokens only contain the most important data of a Chearmyp node. For example, a simplex node can
have no attachers but it cannot exist without the concept name. The list below gives specific
details about each tokens.
- *Scope level*. This token represents the indention of the subsequent token(s).
- *Simplex*. As stated above, a simplex token must contain the concept name only because it is the
  most important data.
- *Complex*. Same with the simplex, the concept name of a complex is the most important because a
  complex node can have no attachers or contents.
- *Attacher*. The label and content of an attacher are both important to add information to the
  simplex or complex they are attaching to.
- *Comment*. Since there are two types of comment nodes, they also have distinction as tokens.
  - *Line comment*. All of its content are important.
  - *Block comment*. The block that the block comment node contains is important.
- *Othertongue*. Othertongue tokens are almost the same as the comment tokens. Their only difference
  is that they are intended to represent a foreign concept.
  - *Line othertongue*
  - *Block othertongue*

## Token Queue Representation
Consider the following Chearmyp text:
```
hello
	name: ABC
	# DEF
```

The token queue will represent the text as series of tokens. It will have a complex token (`hello`),
scope level token (with a value of 1), an attacher token (with `name` label and `ABC` content), and
a line comment token (with a content of ` DEF`) in that order.

## Notes

### License
The repository is licensed under [MIT].

### Want to contribute?
Read the [contributing guide] for different ways to contribute in the project.

### Author
Chearmyp Token was created by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
[MIT]: https://github.com/KennethTrecy/chearmyp_token/blob/master/LICENSE
[Node.js and NPM]: https://nodejs.org/en/
[pnpm]: https://pnpm.io/installation
[Conventional Commits specification]: https://www.conventionalcommits.org/en/v1.0.0/
[contributing guide]: ./CONTRIBUTING.md
[lexer library]: https://github.com/KennethTrecy/chearmyp_lexer
