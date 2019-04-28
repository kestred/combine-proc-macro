# combine-proc-macro

[![combine-proc-macro on docs.rs](https://docs.rs/combine-proc-macro/badge.svg)](https://docs.rs/combine-proc-macro)

A library that allows [proc_macro] function-like macros to be parsed using
the [combine] parser combinator crate.

[proc_macro]: https://doc.rust-lang.org/stable/proc_macro/index.html
[combine]: https://docs.rs/crate/combine

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
combine-proc-macro = "0.2.0"
```

And this in your crate root:

```rust
extern crate combine_proc_macro;
```

To see how the library is used, see the example in the [documentation](https://docs.rs/combine-proc-macro).

## Motivation
When writing a `#[proc_macro_derive]` the input is Rust source code which is
well supported by the `syn` crate.  However, when writing a `#[proc_macro]`
macro, it is common to want to define a custom domain specific language.

This crate allows you to write a parser for your DSL using the `combine`
parser combinator library. It also preserves the source _span_ information
in the parsed result such that `rustc` can provide correct source locations
for identifiers and literals that are re-used in the output.

### License
This library is licensed under the terms of both the MIT license and the Apache License (Version 2.0), and may include packages written by third parties which carry their own copyright notices and license terms.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
