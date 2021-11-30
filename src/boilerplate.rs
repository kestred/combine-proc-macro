#[macro_export]
/// A macro to remove the generics boilerplate when defining parsers.
macro_rules! parser {
    (fn $name:ident() -> $output:ty $block:block) => {
        pub fn $name<I>() -> impl ::combine::Parser<I, Output = $output>
        where
            I: ::combine::Stream<Token = $crate::Token>,
            I::Error: ::combine::ParseError<I::Token, I::Range, I::Position>,
        {
            $block
        }
    };
    (fn $name:ident($input:ident: &mut Input) -> $output:ty $block:block) => {
        pub fn $name<I>(
            $input: &mut I,
        ) -> ::combine::ParseResult<$output, <I as ::combine::StreamOnce>::Error>
        where
            I: ::combine::Stream<Token = $crate::Token>,
            I::Error: ::combine::ParseError<I::Token, I::Range, I::Position>,
        {
            $block
        }
    };
}
