# J2

lumiknit's jump helper 2.

## Objective

- Fuzzy find and jump to the directory
- Clone some repository with less path-conflictions
- Make tagged sandbox & notes

## Installation

- Run `cargo build --release`
- Copy `./target/release/j2` to some bin path which is included in `$PATH`
- Add configuration in shell profile, such as `.bashrc` or `.zshrc`

### Example Configuration

```sh
export HOME=/Users/user
# Repository path, where cloned repositories are stored
export J2_REPOS_PATH="$HOME/repos"
# Root paths to fuzzy find and jump. Multiple paths are separated by colon.
export J2_FIND_BASE_PATHS="$HOME/repos:$HOME/workspace"
# Path list to be ignored during fuzzy find. Multiple paths are separated by colon.
export J2_IGNORES="node_modules:target:dist:venv"
# Path to jones (j-zone, sandbox).
export J2_JONES_PATH="$HOME/workspace/jones"
# Default editor for jones. It'll be used as `$J2_EDITOR <FILENAME>`.
export J2_EDITOR="vi"
# Initialize j2 functions
eval "$(/path/to/j2 init)"
```

Note that the above will apply some commands such as `J`, `j`, `j-`, `j--`, `j.`.

## Usage

### Clone Repository

To clone some repository, run `j2 clone <URL>` or `J clone <URL>`.
It'll create a directory in J2_REPOS_PATH and clone the repository.
For example, `J clone https://github.com/lumiknit/J2` will create `$J2_REPOS_PATH/github.com/lumiknit/J2`.

### Jump

To jump to some directory, run one of `J cd <QUERY>` or `j <QUERY>`.
It'll find a directory in one of J2_FIND_BASE_PATHS, which subpath containing QUERY.

If you want pushd instead of cd, run `J pushd <QUERY>`.

If you already installed `fd` and `fzf`, just type `j` to interactive finding.

### Jones

Jones (J-zone) are a kind of sandbox/playground.
It is sorted by tags and date.
For example, `test-repo/2387-1231` means,

- `test-repo`: a tag (or kind) of the sandbox
- `2387-1231`: Section.
  - `2387`: date. 2023-08-07. Note that the month and day uses base-36.
  - `1231`: random base-36 number to avoid confliction

Note that `_` is used for empty tag.

There are commands to create and use jones:

- `J jone-new [<TAG>]`: Create a new section of jone named with the given tag.
- `J jone-list`: Show all jone tags.
- `J jone-sections [<TAG>]`: Show all sections of jone with the given tag.
- `J jone-note [<TAG>]`: Edit a note of jone with the given tag. The name of the note is `README.md`.

and shortcuts:

- `j--`: Equivalent to `J jone-new`.
- `j-`: Cd to the latest section of jone with the given tag.
- `j_`: Equivalent to `J jone-sections`.
- `j.`: Equivalent to `J jone-note`.

Example usages:

- To manage a todo list,
  - If you want to create a new document, run `j-- todo` to create a new section of jone named `todo`.
  - To edit the document, just run `j. todo` and edit the note.
- To create sandbox directory,
  - Run `j-- s` to create a new section.
  - Run `j- s` to move into the new sandbox.
  - Even if you move to another directory, you can move back to the sandbox by running `j- s`.
  - Run `j_ s` to show all sandboxes.
