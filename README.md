# monkey-interpreter

A Rust implementation of the Monkey programming language, built by following
[*Writing An Interpreter In Go*](https://interpreterbook.com/) by Thorsten
Ball and porting the design from Go to Rust.

Monkey is a small C-like language with integers, booleans, strings, arrays,
hashes, first-class functions, closures, and a handful of built-in functions.
This project implements a lexer, a Pratt parser, and a tree-walking
evaluator, along with a REPL.

## Why Rust instead of Go

The book's design leans heavily on Go interfaces (`Node`, `Expression`,
`Object`) with dynamic dispatch and type assertions. This project reproduces
that shape in Rust using trait objects (`Box<dyn Object>`, `Box<dyn
Expression>`) and `Any` + `downcast_ref`/`downcast` in place of Go's type
switches. A few spots needed a genuinely different approach because Rust's
trait system isn't Go's interface system:

- Cloning a `Box<dyn Trait>` isn't automatic, so `clone_box()` is part of
  the `Object`/`Expression`/`Statement` traits and manual `Clone` impls
  forward to it wherever a struct holds a trait object.
- A `HashMap` key must be `Eq + Hash`, but `dyn Object` can't be (different
  concrete types can't be compared generically). Hash literal keys are
  therefore converted to a small `HashKey` struct via a separate `Hashable`
  trait implemented only for the hashable types (`Integer`, `Boolean`,
  `StringObject`), mirroring the book's `Hashable` interface and its
  `key.(Hashable)` type assertion.

## Project layout

- [`src/token.rs`](src/token.rs) — token types produced by the lexer.
- [`src/lexer.rs`](src/lexer.rs) — turns source text into a stream of tokens.
- [`src/ast.rs`](src/ast.rs) — AST node definitions (`Node`, `Statement`,
  `Expression`) and their `Display` implementations.
- [`src/parser.rs`](src/parser.rs) — a Pratt (top-down operator precedence)
  parser that turns tokens into an AST.
- [`src/object.rs`](src/object.rs) — runtime object system (`Integer`,
  `Boolean`, `StringObject`, `Array`, `HashObject`, `Function`, `Builtin`,
  `Environment`, etc.).
- [`src/evaluator.rs`](src/evaluator.rs) — tree-walking evaluator that
  produces `Object`s from an AST.
- [`src/builtin.rs`](src/builtin.rs) — built-in functions (`len`, `first`,
  `last`, `rest`, `push`).
- [`src/repl.rs`](src/repl.rs) — a read-eval-print loop over stdin/stdout.

## Language features

- Integers, booleans, strings, arrays, and hashes
- Arithmetic, comparison, and string concatenation operators
- `let` bindings and `return`
- `if`/`else` expressions
- First-class functions and closures
- Built-in functions: `len`, `first`, `last`, `rest`, `push`

```monkey
let map = fn(arr, f) {
    let iter = fn(arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), push(accumulated, f(first(arr))));
        }
    };

    iter(arr, []);
};

let double = fn(x) { x * 2 };
map([1, 2, 3, 4], double); // [2, 4, 6, 8]

let people = [{"name": "Alice", "age": 24}, {"name": "Anna", "age": 28}];
people[0]["name"]; // "Alice"
```

## Running

```sh
cargo run       # start the REPL
cargo test      # run the lexer/parser/evaluator test suite
```
