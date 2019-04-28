//! Utilities to generate diagnostic error messages.

use crate::input::Token;
use combine::ParseError;
use combine::stream::StreamOnce;
use proc_macro::{TokenTree, TokenStream};
use std::convert::TryFrom;
use std::fmt;

const DEFAULT_MAX_TRAILING: usize = 50;

/// Incomplete is used in a `#[proc_macro]` to check that all tokens in the Input
/// have been parsed completely or otherwise provide a printable diagnostic-friendly
/// representation of remaining tokens.
///
/// ```rust,ignore
/// let (ast, trailing) = match parse() {
///     Ok(ok) => ok,
///     Err(err) => panic!("error parsing in `my_example` macro: {:#?}", err),
/// };
/// if let Ok(diagnostic) = Incomplete::from_stream(trailing) {
///     panic!("unexpected tokens at end of input:\n\n{}", diagnostic);
/// }
/// ```
#[derive(Debug)]
pub struct Incomplete {
    trailing: Vec<TokenTree>,

    // The maximum number of
    max_trailing: usize,
}

impl Incomplete {
    pub fn from_stream<I>(mut input: I) -> Option<Incomplete>
    where
        I: StreamOnce<Item = Token>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        let mut trailing = Vec::new();
        while let Ok(tok) = input.uncons() {
            trailing.extend(TokenTree::try_from(tok).into_iter());
            if trailing.len() > DEFAULT_MAX_TRAILING {
                break;
            }
        }
        if trailing.len() > 0 {
            Some(Incomplete {
                trailing,
                max_trailing: DEFAULT_MAX_TRAILING,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Incomplete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = self.trailing.len();
        let mut stream = TokenStream::new();
        stream.extend(self.trailing.iter().take(self.max_trailing).cloned());
        if total > self.max_trailing {
            write!(f, "{} [and {} more ...]", stream, total - self.max_trailing)
        } else {
            write!(f, "{}", stream)
        }
    }
}
