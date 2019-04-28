//! A library that allows [proc_macro] function-like macros to be parsed using
//! the [combine] parser combinator crate.
//!
//! [proc_macro]: https://doc.rust-lang.org/stable/proc_macro/index.html
//! [combine]: https://docs.rs/crate/combine
//!
//! ## Motivation
//! When writing `#[proc_macro_derive]` constructions, you parse Rust source
//! code which is well supported by the `syn` crate.  However, when writing a
//! `#[proc_macro]` macro, it is common to define a custom domain specific language.
//!
//! This crate allows you to write a parser for your DSL using the `combine`
//! parser combinator library. It also preserves the source _span_ information
//! in the parsed result such that `rustc` can provide correct source locations
//! for identifiers and literals that are re-used in the output.
//!
//! ## Usage
//! A simple example.
//!
//! ### parser
//! Using a plain combine function.
//!
//! ```rust
//! # extern crate proc_macro;
//! use combine::{ParseError, Parser, Stream};
//! use combine_proc_macro::Token;
//! use combine_proc_macro::parser::{keyword, literal, punct};
//! use proc_macro::Literal;
//!
//! /// Parses expressions like `hello "world"!`.
//! fn hello_grammar<I>() -> impl Parser<Input = I, Output = Literal>
//! where
//!     I: Stream<Item = Token>,
//!     I::Error: ParseError<I::Item, I::Range, I::Position>,
//! {
//!     ( keyword("hello")
//!     , literal()
//!     , punct('!')
//!     ).map(|(_, greeting, _)| greeting)
//! }
//! ```
//!
//! Using the `parser!` macro can help remove boilerplate.
//!
//! ```rust
//! # extern crate proc_macro;
//! use combine::Parser;
//! use combine_proc_macro::parser;
//! use combine_proc_macro::parser::{keyword, literal, punct};
//! use proc_macro::Literal;
//!
//! parser!(fn hello_grammar() -> Literal {
//!     ( keyword("hello")
//!     , literal()
//!     , punct('!')
//!     ).map(|(_, greeting, _)| greeting)
//! });
//! ```
//!
//! ### proc_macro
//!
//! ```ignore
//! extern crate proc_macro;
//! use combine::parser::Parser;
//! use combine_proc_macro::{Input, Incomplete};
//! use proc_macro::TokenStream;
//!
//! #[proc_macro]
//! pub fn hello_macro(input: TokenStream) -> TokenStream {
//!     let input: Input = input.into();
//!
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
//! #
//! # fn hello_grammar<I>() -> impl Parser<Input = I, Output = Literal>
//! # where
//! #     I: Stream<Item = Token>,
//! #     I::Error: ParseError<I::Item, I::Range, I::Position> { unimplemented!() }
//! #
//! # fn impl_hello_macro(ast: Literal) -> TokenStream { unimplemented!() }
//! ```

extern crate proc_macro;

mod boilerplate;
mod diagnostic;
pub mod input;
pub mod parser;

pub use diagnostic::Incomplete;
pub use input::{Input, Token};
