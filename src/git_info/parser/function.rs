use std::str;

use nom::{IResult, Err, ErrorKind, alphanumeric};

use super::{ParseExpression};
use parser::color::TerminalColor;

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
named!(function<&[u8], (&[u8], Vec<&[u8]>)>, delimited!(
    char!('{'),
    tuple!(
        take_until!("("),
        params
    ),
    char!('}')
));

// Convert a byte slice to a number
// TODO: clean up cusom errors
fn slice_to_num<T>(input: &[u8]) -> IResult<&[u8], T>
where T: str::FromStr {
    // Convert bytes to string
    match str::from_utf8(input) {
        Ok(string) => {
            match str::parse::<T>(string) {
                Ok(num) => IResult::Done(&b""[..], num),
                Err(_) => {
                    IResult::Error(
                        Err::Position(
                            ErrorKind::Custom(0),
                            input
                        )
                    )
                }
            }
        },
        Err(_) => {
            IResult::Error(
                Err::Position(
                    ErrorKind::Custom(0),
                    input
                )
            )
        }
    }
}

pub fn function_parser(input: &[u8]) -> IResult<&[u8], ParseExpression> {
    let (remaining, info) = try_parse!(input, function);
    match info {
        // Color function
        (b"color", ref params) if params.len() == 1 => {
            IResult::Done(
                remaining,
                ParseExpression::Color(
                    TerminalColor::convert(params[0])
                )
            )
        },
        // TrueColor
        (b"rgb", ref params) if params.len() == 3 => {
            let (_, red) = try_parse!(params[0], slice_to_num);
            let (_, green) = try_parse!(params[1], slice_to_num);
            let (_, blue) = try_parse!(params[2], slice_to_num);
            IResult::Done(
                remaining,
                ParseExpression::Color(
                    TerminalColor::TrueColor(red, green, blue)
                )
            )
        },
        // Unrecoganized function
        (&_, _) => {
            IResult::Error(
                Err::Position(
                    ErrorKind::Custom(0),
                    input
                )
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use parser::{ParseExpression};
    use parser::color::TerminalColor;

    use super::{function_parser, params, function, slice_to_num};

    // Test the params parser

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

    // Test the function parser

    #[test]
    fn parse_function() {
        assert_eq!(
            function(b"{test(haha,hello)}"),
            IResult::Done(
                &b""[..],
                (&b"test"[..], vec![&b"haha"[..], &b"hello"[..]])
            )
        );
    }

    #[test]
    fn parse_function_spaced_commas() {
        assert_eq!(
            function(b"{test(haha, hello)}"),
            IResult::Done(
                &b""[..],
                (&b"test"[..], vec![&b"haha"[..], &b"hello"[..]])
            )
        );
    }

    // Test the function to expression conversion

    /// Test one of each function
    #[test]
    fn one_of_each() {
        assert_eq!(
            function_parser(b"{color(red)}"),
            IResult::Done(
                &b""[..],
                ParseExpression::Color(TerminalColor::Red)
            )
        );

        assert_eq!(
            function_parser(b"{rgb(255, 0, 255)}"),
            IResult::Done(
                &b""[..],
                ParseExpression::Color(TerminalColor::TrueColor(255, 0, 255))
            )
        );
    }

    // Test util functions
    #[test]
    fn slice_to_num_conversion() {
        assert_eq!(
            slice_to_num(&b"122"[..]),
            IResult::Done(
                &b""[..],
                122u8
            )
        );
    }
}
