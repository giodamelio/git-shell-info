pub mod color;
mod variable;
mod function;

use std::str;

use nom;
use nom::IResult;

use super::errors::GitInfoError;
use self::variable::variable_parser;
use self::function::function_parser;

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

// One or more expressions seperated some string literals
named!(expressions<&[u8], Vec<ParseExpression> >, many0!(alt!(
    function_parser |
    variable_parser |
    map!(
        map_res!(
            take_while!(brace_or_eol),
            str::from_utf8
        ),
        ParseExpression::Literal
    )
)));

// TODO: clean up incomplete handeling
pub fn parse(template: &str) -> Result<Vec<ParseExpression>, GitInfoError> {
    match expressions(template.as_bytes()) {
        IResult::Done(_, result) => Ok(result),
        IResult::Error(err) =>
            Err(GitInfoError::ParseError(err)),
        IResult::Incomplete(_) =>
            Err(GitInfoError::ParseError(
                nom::Err::Code(nom::ErrorKind::Custom(0)))),
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::{ParseExpression, expressions};
    use super::color::TerminalColor;

    #[test]
    fn three_items() {
        assert_eq!(
            expressions(b"{color(red)}{commit_count}{branch}"),
            IResult::Done(
                &b""[..],
                vec![
                    ParseExpression::Color(TerminalColor::Red),
                    ParseExpression::CommitCount,
                    ParseExpression::Branch
                ],
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
}
