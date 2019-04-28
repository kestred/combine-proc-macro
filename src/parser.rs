use crate::input::Token;
use combine::{ParseError, Parser, Stream, StreamOnce};
use combine::error::{ConsumedResult, FastResult::*, Info, Tracked};
use combine::stream::uncons;
use std::marker::PhantomData;

pub fn ident<I>() -> Ident<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    Ident(PhantomData)
}

#[derive(Copy, Clone)]
pub struct Ident<I>(PhantomData<fn(I) -> I>)
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>;

impl<I> Parser for Ident<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    type Input = I;
    type Output = proc_macro::Ident;
    type PartialState = ();

    fn parse_lazy(&mut self, input: &mut Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        let position = input.position();
        match uncons(input) {
            EmptyOk(tok) | ConsumedOk(tok) => match tok {
                Token::Ident(ident) => ConsumedOk(ident),
                _ => EmptyErr(I::Error::empty(position).into()),
            },
            EmptyErr(err) => EmptyErr(err),
            ConsumedErr(err) => ConsumedErr(err),
        }
    }

    fn add_error(&mut self, errors: &mut Tracked<<Self::Input as StreamOnce>::Error>) {
        errors.error.add_expected(Info::Borrowed("<Ident>"));
    }
}

pub fn keyword<I>(word: &'static str) -> Keyword<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    Keyword(word, PhantomData)
}

#[derive(Copy, Clone)]
pub struct Keyword<I>(&'static str, PhantomData<fn(I) -> I>)
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>;

impl<I> Parser for Keyword<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    type Input = I;
    type Output = Token;
    type PartialState = ();

    fn parse_lazy(&mut self, input: &mut Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        let position = input.position();
        match uncons(input) {
            EmptyOk(tok) | ConsumedOk(tok) => match tok {
                Token::Ident(ref ident) if ident.to_string() == self.0 => ConsumedOk(tok),
                _ => EmptyErr(I::Error::empty(position).into()),
            },
            EmptyErr(err) => EmptyErr(err),
            ConsumedErr(err) => ConsumedErr(err),
        }
    }

    fn add_error(&mut self, errors: &mut Tracked<<Self::Input as StreamOnce>::Error>) {
        errors.error.add_expected(Info::Borrowed(self.0));
    }
}

pub fn literal<I>() -> Literal<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    Literal(PhantomData)
}

#[derive(Copy, Clone)]
pub struct Literal<I>(PhantomData<fn(I) -> I>)
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>;

impl<I> Parser for Literal<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    type Input = I;
    type Output = proc_macro::Literal;
    type PartialState = ();

    fn parse_lazy(&mut self, input: &mut Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        let position = input.position();
        match uncons(input) {
            EmptyOk(tok) | ConsumedOk(tok) => match tok {
                Token::Literal(lit) => ConsumedOk(lit),
                _ => EmptyErr(I::Error::empty(position).into()),
            },
            EmptyErr(err) => EmptyErr(err),
            ConsumedErr(err) => ConsumedErr(err),
        }
    }

    fn add_error(&mut self, errors: &mut Tracked<<Self::Input as StreamOnce>::Error>) {
        errors.error.add_expected(Info::Borrowed("<Literal>"));
    }
}

pub fn punct<I>(c: char) -> Punct<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    Punct(c, PhantomData)
}

#[derive(Copy, Clone)]
pub struct Punct<I>(char, PhantomData<fn(I) -> I>)
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>;

impl<I> Parser for Punct<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    type Input = I;
    type Output = Token;
    type PartialState = ();

    fn parse_lazy(&mut self, input: &mut Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        let position = input.position();
        match uncons(input) {
            EmptyOk(tok) | ConsumedOk(tok) => match tok {
                Token::Punct(ref punct) if punct.as_char() == self.0 => ConsumedOk(tok),
                _ => EmptyErr(I::Error::empty(position).into()),
            },
            EmptyErr(err) => EmptyErr(err),
            ConsumedErr(err) => ConsumedErr(err),
        }
    }

    fn add_error(&mut self, errors: &mut Tracked<<Self::Input as StreamOnce>::Error>) {
        errors.error.add_expected(Info::Borrowed("<Punct>"));
    }
}

pub fn delim<I>(c: char) -> Delim<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    Delim(c, PhantomData)
}

#[derive(Copy, Clone)]
pub struct Delim<I>(char, PhantomData<fn(I) -> I>)
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>;

impl<I> Parser for Delim<I>
where
    I: Stream<Item = Token>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    type Input = I;
    type Output = Token;
    type PartialState = ();

    fn parse_lazy(&mut self, input: &mut Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        let position = input.position();
        match uncons(input) {
            EmptyOk(tok) | ConsumedOk(tok) => match tok {
                Token::Delim(ch, _) if ch == self.0 => ConsumedOk(tok),
                _ => EmptyErr(I::Error::empty(position).into()),
            },
            EmptyErr(err) => EmptyErr(err),
            ConsumedErr(err) => ConsumedErr(err),
        }
    }

    fn add_error(&mut self, errors: &mut Tracked<<Self::Input as StreamOnce>::Error>) {
        errors.error.add_expected(Info::Borrowed("<Delim>"));
    }
}
