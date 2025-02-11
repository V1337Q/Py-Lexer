# Py-Lexer

A simple lexer for Python written in Rust.

## Usage

To run the lexer, insert your Python syntax inside the Input variable, and simply run the following command:

```bash
cargo run
```

This will print the tokens of the input string.

## Example

Input:

```python
def add(a, b):
    return a + b
```

Output:

```
Keyword(def)
Identifier(add)
Punctuation(()
Identifier(a)
Punctuation(,)
Identifier(b)
Punctuation())
Punctuation(:)
Keyword(return)
Identifier(a)
Operator(+)
Identifier(b)
Punctuation()
```


