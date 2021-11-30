//! Wrappers and transforms to around `proc_macro` types to implement `combine` traits.

use combine::stream::easy::Error;
use combine::stream::{ResetStream, StreamErrorFor};
use combine::{Positioned, StreamOnce};
use proc_macro::TokenStream as TokenStreamBuiltin;
use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Input {
    source_stack: Vec<(IntoIter, Option<Token>)>,
    source_pos: usize,
}

impl Input {
    /// Returns `true` if the input contains no more tokens.
    pub fn is_empty(&self) -> bool {
        self.source_stack.is_empty()
    }

    fn next(&mut self) -> Option<Token> {
        if self.source_stack.is_empty() {
            return None;
        }

        while !self.source_stack.is_empty() {
            let next = self
                .source_stack
                .last_mut()
                .and_then(|(iter, _)| iter.next());
            let next = match next {
                Some(tt) => self.ungroup(tt),
                None => None,
            };
            if let Some(tok) = next {
                return Some(tok);
            }
            let (_empty, close) = self.source_stack.pop().unwrap();
            if let Some(tok) = close {
                return Some(tok);
            }
        }

        // We're finally at the end of input Bob...
        None
    }

    fn ungroup(&mut self, tt: TokenTree) -> Option<Token> {
        match tt {
            TokenTree::Punct(tok) => Some(Token::Punct(tok)),
            TokenTree::Ident(tok) => Some(Token::Ident(tok)),
            TokenTree::Literal(tok) => Some(Token::Literal(tok)),
            TokenTree::Group(group) => {
                let (open, close) = match group.delimiter() {
                    Delimiter::Parenthesis => (Some('('), Some(')')),
                    Delimiter::Brace => (Some('{'), Some('}')),
                    Delimiter::Bracket => (Some('['), Some(']')),
                    Delimiter::None => (None, None),
                };
                self.source_stack.push((
                    group.stream().into_iter(),
                    close.map(|c| Token::Delim(c, group.span())),
                ));
                match open {
                    Some(c) => Some(Token::Delim(c, group.span())),
                    None => None,
                }
            }
        }
    }
}
impl From<TokenStreamBuiltin> for Input {
    fn from(stream: TokenStreamBuiltin) -> Input {
        let stream: TokenStream = stream.to_string().parse().unwrap();
        Input {
            source_stack: vec![(stream.into_iter(), None)],
            source_pos: 0,
        }
    }
}
impl From<TokenStream> for Input {
    fn from(stream: TokenStream) -> Input {
        Input {
            source_stack: vec![(stream.into_iter(), None)],
            source_pos: 0,
        }
    }
}
impl From<Input> for TokenStream {
    fn from(input: Input) -> TokenStream {
        let mut rem = TokenStream::new();
        for (source, close) in input.source_stack.into_iter().rev() {
            rem.extend(source);
            rem.extend(
                close
                    .into_iter()
                    .map(|tok| TokenTree::try_from(tok).unwrap()),
            );
        }
        rem
    }
}

impl StreamOnce for Input {
    type Token = Token;
    type Range = Self::Token;
    type Position = usize;
    type Error = Error<Self::Token, Self::Range>;

    fn uncons(&mut self) -> Result<Self::Token, StreamErrorFor<Self>> {
        match self.next() {
            None => Err(Error::end_of_input()),
            Some(tok) => {
                self.source_pos += 1;
                Ok(tok)
            }
        }
    }

    fn is_partial(&self) -> bool {
        false
    }
}

impl Positioned for Input {
    fn position(&self) -> Self::Position {
        self.source_pos
    }
}

impl ResetStream for Input {
    type Checkpoint = Self;

    fn checkpoint(&self) -> Self::Checkpoint {
        self.clone()
    }

    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        *self = checkpoint;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SpanPosition {
    pos: usize,
    span: Span,
}

impl SpanPosition {
    pub fn into_span(&self) -> Span {
        self.span
    }
}

impl Default for SpanPosition {
    fn default() -> Self {
        SpanPosition {
            pos: 0,
            span: Span::call_site(),
        }
    }
}

impl PartialOrd for SpanPosition {
    fn partial_cmp(&self, other: &SpanPosition) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpanPosition {
    fn cmp(&self, other: &SpanPosition) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialEq for SpanPosition {
    fn eq(&self, other: &SpanPosition) -> bool {
        self.pos == other.pos
    }
}

impl Eq for SpanPosition {}

#[derive(Clone, Debug)]
pub enum Token {
    Delim(char, Span),
    Punct(Punct),
    Ident(Ident),
    Literal(Literal),
}

impl Token {
    pub fn to_char(&self) -> Option<char> {
        match self {
            Token::Delim(ch, _) => Some(*ch),
            Token::Punct(punct) => Some(punct.as_char()),
            _ => None,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Token::Delim(_, span) => span.clone(),
            Token::Punct(tok) => tok.span(),
            Token::Ident(tok) => tok.span(),
            Token::Literal(tok) => tok.span(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Delim(l, _), Token::Delim(r, _)) => l == r,
            (Token::Punct(l), Token::Punct(r)) => l.as_char() == r.as_char(),
            (Token::Ident(l), Token::Ident(r)) => l.to_string() == r.to_string(),
            (Token::Literal(l), Token::Literal(r)) => l.to_string() == r.to_string(),
            _ => false,
        }
    }
}

impl TryFrom<Token> for TokenTree {
    type Error = ();

    fn try_from(tok: Token) -> Result<TokenTree, Self::Error> {
        match tok {
            Token::Delim(_, _) => Err(()),
            Token::Punct(tok) => Ok(TokenTree::Punct(tok)),
            Token::Ident(tok) => Ok(TokenTree::Ident(tok)),
            Token::Literal(tok) => Ok(TokenTree::Literal(tok)),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Delim(l, _) => write!(f, "{}", l),
            Token::Punct(l) => write!(f, "{}", l.as_char()),
            Token::Ident(l) => write!(f, "{}", l),
            Token::Literal(l) => write!(f, "{}", l),
        }
    }
}
