[![Workflow Status](https://github.com/FauconFan/mdbook-cmdrun/actions/workflows/main.yml/badge.svg)](https://github.com/FauconFan/mdbook-cmdrun/actions?query=workflow%3A%22main%22)
![Crates.io](https://img.shields.io/crates/l/mdbook-cmdrun)

# mdbook-cmdrun

This is a preprocessor for the [rust-lang mdbook](https://github.com/rust-lang/mdBook) project. This allows to run arbitrary commands and include the output of these commands within the readme file.

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

Current version: 0.1.0  
License: MIT
