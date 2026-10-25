# rlox

A Rust implementation of **Lox**, the scripting language from Robert Nystrom's
[*Crafting Interpreters*](https://craftinginterpreters.com/). It follows the book's
tree-walking interpreter (jlox), translated from Java into Rust.

> **Status: work in progress.** The lexer is done; the parser and interpreter are next.

## Progress

- [x] Scanning (lexer)
- [ ] Representing code (AST)
- [ ] Parsing expressions
- [ ] Evaluating expressions
- [ ] Statements and state
- [ ] Control flow
- [ ] Functions and closures
- [ ] Resolving and binding
- [ ] Classes and inheritance

## Building and running

You need a Rust toolchain (edition 2024). If you use Nix, `flake.nix` provides a dev
shell (`nix develop`, or `direnv allow` with the included `.envrc`).

```sh
cargo build
cargo run -- path/to/script.lox
```

## Testing

```sh
cargo test
```

- **Unit tests** are in `unittest/`, not inline in `src/`. Each source file pulls in
  its tests with a `#[path]` module, so they can still reach private items:

  ```rust
  #[cfg(test)]
  #[path = "../unittest/lexer_test.rs"]
  mod tests;
  ```

- **`test_cases/`** contains the `.lox` programs from the official
  [Crafting Interpreters test suite](https://github.com/munificent/craftinginterpreters/tree/master/test).
  The plan is to run the whole suite against rlox as later chapters are finished.

## Design notes: Rust vs. the book

jlox is written in Java and relies on things Rust doesn't have, such as `Object`,
`null`, method overloading and a garbage collector. These are the places where the
Rust version differs from the book, grouped by the part of the interpreter they
belong to.

### Scanning (lexer)

#### Literals: `Object` -> `enum`

The book stores a token's literal value as a Java `Object`. I first considered a Rust
`union`, but a union doesn't record which field is active, so every read is `unsafe`.
It also can't easily hold a `String`. A Rust `enum` is a tagged union that the
compiler checks:

```rust
pub enum Literal {
    String(String),
    Number(f64),   // all Lox numbers are doubles
    Bool(bool),
    Nil,
}
```

`Token.literal` is an `Option<Literal>`, where `None` means "this token has no
literal". That's clearer than using `null` both for "no literal" and for Lox's `nil`.

#### No overloading, one `add_token`

The book has two versions of `addToken`, one with a literal and one without. Rust has
no function overloading, so there's a single `add_token(token_type, Option<Literal>)`.
Each scanning routine builds its own `Literal`, because it already knows what it just
scanned. That replaced my first attempt, which tried to pick a type generically.

#### `peek()` returns `Option<u8>`

The book's `peek()` returns `'\0'` at the end of input. Here it returns
`Option<u8>`, written as `self.source.get(i).copied()`, so every caller has to handle
end of input explicitly.

#### Bytes, not `char`s

The scanner works on the source as a `Vec<u8>`, and lexemes are slices with
half-open ranges (`source[start..cursor]`). That's simple and fast for Lox's ASCII
syntax.

## Acknowledgements

The language design, the reference implementation and the test programs in
`test_cases/` are by Robert Nystrom, from
[munificent/craftinginterpreters](https://github.com/munificent/craftinginterpreters)
(MIT licensed).
