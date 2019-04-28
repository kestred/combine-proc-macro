#[macro_export]
macro_rules! parser {
    (fn $name:ident() -> $output:ty $block:block) => {
        pub fn $name<I>() -> impl ::combine::Parser<Input = I, Output = $output>
        where
            I: ::combine::Stream<Item = $crate::Token>,
            I::Error: ::combine::ParseError<I::Item, I::Range, I::Position>,
        {
            $block
        }
    };
    (fn $name:ident($input:ident: &mut Input) -> $output:ty $block:block) => {
        pub fn $name<I>($input: &mut I) -> ::combine::ParseResult<$output, I>
        where
            I: ::combine::Stream<Item = $crate::Token>,
            I::Error: ::combine::ParseError<I::Item, I::Range, I::Position>,
        {
            $block
        }
    };
}