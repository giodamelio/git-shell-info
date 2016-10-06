pub mod color;
mod variable;

use std::str;

use nom::IResult;

use super::errors::GitInfoError;
use self::variable::variable_parser;

/// Go until there is a {
fn brace_or_eol(char: u8) -> bool {
    char != b'{'
}

#[test]
fn brace_or_eol_test() {
    assert_eq!(brace_or_eol(b'{'), false);
    assert_eq!(brace_or_eol(b'A'), true);
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChangeType {
    // Working tree changes
    WTNew,
    WTModified,
    WTDeleted,
    WTRenamed,
    WTTypechange,

    // Staged changes
    StagedNew,
    StagedModified,
    StagedDeleted,
    StagedRenamed,
    StagedTypechange,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseExpression<'a> {
    Literal(&'a str),
    Branch,
    CommitCount,
    ChangeCount(ChangeType),
    Color(color::TerminalColor),
}

// Parse a function call with a single parameter
named!(single_param<&[u8], &[u8]>, delimited!(
    char!('('),
    take_until!(")"),
    char!(')')
));

// A function with some parameters
named!(function<&[u8], ParseExpression>, delimited!(
    char!('{'),
    alt!(
        tuple!(
            tag!("color"),
            single_param
        ) => { |params: (&[u8], &[u8])| ParseExpression::Color(color::TerminalColor::convert(params.1)) }
    ),
    char!('}')
));

// One or more expressions seperated some string literals
named!(expressions<&[u8], Vec<ParseExpression> >, many0!(alt!(
    function |
    variable_parser |
    map!(
        map_res!(
            take_while!(brace_or_eol),
            str::from_utf8
        ),
        ParseExpression::Literal
    )
)));

pub fn parse(template: &str) -> Result<Vec<ParseExpression>, GitInfoError> {
    match expressions(template.as_bytes()) {
        IResult::Done(_, result) => Ok(result),
        IResult::Error(_) | IResult::Incomplete(_) => Err(GitInfoError::ParseError),
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::{ParseExpression, ChangeType, expressions, variable, single_param, function};
    use super::color::TerminalColor;

       #[test]
    fn single_function() {
        assert_eq!(function(b"{color(red)}"), IResult::Done(&b""[..], ParseExpression::Color(TerminalColor::Red)));
    }

    #[test]
    fn three_items() {
        assert_eq!(
            expressions(b"{branch}{commit_count}{branch}"),
            IResult::Done(
                &b""[..],
                vec![ParseExpression::Branch, ParseExpression::CommitCount, ParseExpression::Branch],
            )
        );
    }

    #[test]
    fn three_items_with_literals() {
        assert_eq!(
            expressions(b"{branch}|{branch}|{commit_count}"),
            IResult::Done(
                &b""[..],
                vec![
                    ParseExpression::Branch,
                    ParseExpression::Literal("|"),
                    ParseExpression::Branch,
                    ParseExpression::Literal("|"),
                    ParseExpression::CommitCount,
                ],
            )
        );
    }

    #[test]
    fn items_with_literals_at_end() {
        assert_eq!(
            expressions(b"{branch}YAY"),
            IResult::Done(
                &b""[..],
                vec![ParseExpression::Branch, ParseExpression::Literal("YAY")],
            )
        );
    }

    #[test]
    fn parse_single_param() {
        assert_eq!(single_param(b"(red)"), IResult::Done(&b""[..], &b"red"[..]));
    }
}
