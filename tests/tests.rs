#![cfg(test)]
use combine::Parser;
use combine_proc_macro::{parser::*, *};
use proc_macro2::{Delimiter, TokenStream};
use std::{fmt::Debug, str::FromStr};

fn make_input(text: &str) -> Input {
    Input::from(TokenStream::from_str(text).unwrap())
}

fn parse_output<P>(mut parser: P, input: &'static str) -> P::Output
where
    P: Parser<Input>,
{
    parser
        .parse(make_input(input))
        .unwrap_or_else(|err| panic!("{}", err))
        .0
}

fn parse_ok<P>(parser: P, input: &'static str) -> String
where
    P: Parser<Input>,
    P::Output: ToString,
{
    parse_output(parser, input).to_string()
}

fn parse_err<P>(mut parser: P, input: &'static str) -> String
where
    P: Parser<Input>,
    P::Output: Debug,
{
    parser.parse(make_input(input)).unwrap_err().to_string()
}

fn group<P>(delimiter: Delimiter, p: P) -> impl Parser<Input, Output = P::Output>
where
    P: Parser<Input>,
{
    let (open, close) = match delimiter {
        Delimiter::Parenthesis => ('(', ')'),
        Delimiter::Brace => ('{', '}'),
        Delimiter::Bracket => ('[', ']'),
        Delimiter::None => panic!(),
    };
    delim(open).with(p).skip(delim(close))
}

#[test]
fn test_ident() {
    assert_eq!(parse_ok(ident(), "foo+"), "foo");
    assert_eq!(parse_err(ident(), "+"), "Expected `IDENT`");
}

#[test]
fn test_keyword() {
    assert_eq!(parse_ok(keyword("foo"), "foo"), "foo");
    assert_eq!(parse_err(keyword("foo"), "bar"), "Expected `foo`");
}

#[test]
fn test_punct() {
    assert_eq!(parse_ok(punct('+'), "+ +"), "+",);
    assert_eq!(parse_ok(punct('+'), "++"), "+",);
}

#[test]
fn test_delim() {
    assert_eq!(parse_ok(group(Delimiter::Bracket, ident()), "[foo]"), "foo",);
    assert_eq!(parse_ok(group(Delimiter::Brace, ident()), "{foo}"), "foo",);
    assert_eq!(
        parse_ok(group(Delimiter::Parenthesis, ident()), "(foo)"),
        "foo",
    );
    assert_eq!(
        parse_err(group(Delimiter::Brace, ident()), "{\nfoo +\n}"),
        "Expected `}`",
    );
    assert_eq!(
        parse_err(group(Delimiter::Brace, ident()), "{\n+ foo\n}"),
        "Expected `IDENT`",
    );
}

#[test]
#[should_panic]
fn test_invalid_delim() {
    delim::<Input>('+');
}
