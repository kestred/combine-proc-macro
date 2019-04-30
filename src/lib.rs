//! A library that allows [proc_macro] function-like macros to be parsed using
//! the [combine] parser combinator crate.
//!
//! [proc_macro]: https://doc.rust-lang.org/stable/proc_macro/index.html
//! [combine]: https://docs.rs/crate/combine
//!
//! ## Motivation
//! When writing a `#[proc_macro_derive]` the input is Rust source code which is
//! well supported by the `syn` crate.  However, when writing a `#[proc_macro]`
//! macro, it is common to want to define a custom domain specific language.
//!
//! This crate allows you to write a parser for your DSL using the `combine`
//! parser combinator library. It also preserves the source _span_ information
//! in the parsed result such that `rustc` can provide correct source locations
//! for identifiers and literals that are re-used in the output.
//!
//! ## Implementing a parser
//! This is a basic example using base `combine` parsers.
//!
//! ```rust
//! # extern crate proc_macro;
//! use combine::{ParseError, Parser, Stream};
//! use combine_proc_macro::{Token, Literal};
//! use combine_proc_macro::parser::{delim, keyword, literal, punct};
//!
//! /// Parses expressions like `{ hello "world"! }`.
//! fn hello_grammar<I>() -> impl Parser<Input = I, Output = Literal>
//! where
//!     I: Stream<Item = Token>,
//!     I::Error: ParseError<I::Item, I::Range, I::Position>,
//! {
//!     ( delim('{')
//!     , keyword("hello")
//!     , literal()
//!     , punct('!')
//!     , delim('}')
//!     ).map(|(_, _, greeting, _, _)| greeting)
//! }
//! ```
//!
//! Using the `parser!` macro can help remove boilerplate.
//!
//! ```rust
//! # extern crate proc_macro;
//! use combine::Parser;
//! use combine_proc_macro::Literal;
//! use combine_proc_macro::parser;
//! use combine_proc_macro::parser::{delim, keyword, literal, punct};
//!
//! parser!(fn hello_grammar() -> Literal {
//!     ( delim('{')
//!     , keyword("hello")
//!     , literal()
//!     , punct('!')
//!     , delim('}')
//!     ).map(|(_, _, greeting, _, _)| greeting)
//! });
//! ```
//!
//! ## Implementing a macro
//! A proc macro must be defined at the crate root within the `lib.rs` file.
//!
//! ```rust,ignore
//! extern crate proc_macro;
//!
//! use combine::parser::Parser;
//! use combine_proc_macro::{Input, Incomplete};
//! use proc_macro::TokenStream;
//!
//! #[proc_macro]
//! pub fn hello_macro(input: TokenStream) -> TokenStream {
//!     let input = Input::from(input).with_lookahead(1);
//!     let result = hello_grammar().easy_parse(input);
//!     let (ast, trailing) = match result {
//!         Ok(ok) => ok,
//!         Err(err) => panic!("error parsing in `hello_macro` macro: {:#?}", err),
//!     };
//!     if let Some(diagnostic) = Incomplete::from_stream(trailing) {
//!         panic!("unexpected tokens at end of input:\n\n{}", diagnostic);
//!     }
//!
//!     impl_hello_macro(&ast)  // generate rust output; e.g. using the `quote` crate
//! }
//!
//! # use combine::{ParseError, Stream};
//! # use combine_proc_macro::Token;
//! # use combine_proc_macro::parser::literal;
//! # use proc_macro::Literal;
//! #
//! # fn hello_grammar<I>() -> impl Parser<Input = I, Output = Literal>
//! # where
//! #     I: Stream<Item = Token>,
//! #     I::Error: ParseError<I::Item, I::Range, I::Position> { literal() }
//! #
//! # fn impl_hello_macro(ast: &Literal) -> TokenStream { unimplemented!() }
//! ```

extern crate proc_macro;
extern crate proc_macro2;

mod boilerplate;
pub mod diagnostic;
pub mod input;
pub mod parser;

pub use diagnostic::Incomplete;
pub use input::{Input, Token};
pub use proc_macro2::{Ident, Literal, Punct};