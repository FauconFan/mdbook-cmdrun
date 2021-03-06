[![Workflow Status](https://github.com/FauconFan/mdbook-cmdrun/actions/workflows/main.yml/badge.svg)](https://github.com/FauconFan/mdbook-cmdrun/actions?query=workflow%3A%22main%22)
![Crates.io](https://img.shields.io/crates/l/mdbook-cmdrun)

# mdbook-cmdrun

This is a preprocessor for the [rust-lang mdbook](https://github.com/rust-lang/mdBook) project. This allows to run arbitrary (shell) commands and include the output of these commands within the markdown file.

## Getting started

```sh
cargo install mdbook-cmdrun
```

You also have to activate the preprocessor, put this in your `book.toml` file:
```toml
[preprocessor.cmdrun]
```

## How to

Let's say we have these two files:

Markdown file: file.md
```markdown
# Title

<!-- cmdrun seq 1 10 -->

<!-- cmdrun python3 script.py -->

```

Python file: script.py
```python
def main():
    print("## Generated subtitle")
    print("  This comes from the script.py file")
    print("  Since I'm at in a scripting language,")
    print("  I can compute whatever I want")

if __name__ == "__main__":
    main()

```

The preprocessor will call seq then python3, and will produce the resulting file:

```markdown
# Title

1
2
3
4
5
6
7
8
9
10

## Generated subtitle
  This comes from the script.py file
  Since I'm at in a scripting language,
  I can compute whatever I want


```

## Details

When the pattern `<!-- cmdrun $1 -->\n` is encountered, the command `$1` will be run using the shell `sh` like this: `sh -c $1`.
Also the working directory is the directory where the pattern was found (not root).
Any command that takes no input, but a list of command lines arguments and produce output in stdout, stderr is ignored.

## Examples

The following is valid:

````markdown

<!-- runcmd python3 generate_table.py -->

```rust
<!-- runcmd cat program.rs -->
```

```diff
<!-- runcmd diff a.rs b.rs -->
```

```console
<!-- runcmd ls -l . -->
```
````

Some more examples are implemented, and are used as regression tests. You can find them [here](https://github.com/FauconFan/mdbook-cmdrun/tree/master/tests/regression/).
At the moment of writing, there are examples using:
- Shell
- Bash script
- Python3
- Rust


Current version: 0.2.0  
License: MIT
