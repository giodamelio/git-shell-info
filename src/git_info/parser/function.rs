use nom::alphanumeric;

use super::{ParseExpression};
use parser::color::TerminalColor;

// Parse a function call with a single parameter
named!(single_param<&[u8], &[u8]>, delimited!(
    char!('('),
    take_until!(")"),
    char!(')')
));

// Parse a list of parameters
named!(params<&[u8], Vec<&[u8]> >, delimited!(
    char!('('),
    separated_list!(
        terminated!(
            char!(','),
            many0!(char!(' '))
        ),
        alphanumeric
    ),
    char!(')')
));

// A function with some parameters
named!(pub function_parser<&[u8], ParseExpression>, delimited!(
    char!('{'),
    alt!(
        tuple!(
            tag!("color"),
            single_param
        ) => {
            |params: (&[u8], &[u8])|
            ParseExpression::Color(
                TerminalColor::convert(params.1)
            )
        }
    ),
    char!('}')
));

#[cfg(test)]
mod tests {
    use nom::IResult;

    use parser::{ParseExpression};
    use parser::color::TerminalColor;

    use super::{function_parser, single_param, params};

    #[test]
    fn single_function() {
        assert_eq!(
            function_parser(b"{color(red)}"),
            IResult::Done(
                &b""[..],
                ParseExpression::Color(TerminalColor::Red)
            )
        );
    }

    #[test]
    fn parse_single_param() {
        assert_eq!(
            single_param(b"(red)"),
            IResult::Done(&b""[..], &b"red"[..])
        );
    }

    #[test]
    fn parse_params() {
        assert_eq!(
            params(b"(red,yellow,test42)"),
            IResult::Done(&b""[..], vec![
                &b"red"[..],
                &b"yellow"[..],
                &b"test42"[..],
            ])
        );
    }

    #[test]
    fn parse_params_with_spaces() {
        assert_eq!(
            params(b"(red, yellow,      test42)"),
            IResult::Done(&b""[..], vec![
                &b"red"[..],
                &b"yellow"[..],
                &b"test42"[..],
            ])
        );
    }

    #[test]
    fn parse_params_empty() {
        assert_eq!(
            params(b"()"),
            IResult::Done(&b""[..], vec![])
        );
    }
}
