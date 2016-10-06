pub mod color;

use std::str;

use nom::IResult;

use super::errors::GitInfoError;

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

// A single variable
named!(variable<&[u8], ParseExpression>, delimited!(
    char!('{'),
    alt_complete!(
        tag!("branch") => { |_| ParseExpression::Branch } |
        tag!("commit_count") => { |_| ParseExpression::CommitCount } |

        // Changes in the working tree
        tag!("new_count") => { |_| ParseExpression::ChangeCount(ChangeType::WTNew) } |
        tag!("modified_count") => { |_| ParseExpression::ChangeCount(ChangeType::WTModified) } |
        tag!("deleted_count") => { |_| ParseExpression::ChangeCount(ChangeType::WTDeleted) } |
        tag!("renamed_count") => { |_| ParseExpression::ChangeCount(ChangeType::WTRenamed) } |
        tag!("typechange_count") => { |_| ParseExpression::ChangeCount(ChangeType::WTTypechange) } |

        // Stashed changes
        tag!("staged_new_count") => { |_| ParseExpression::ChangeCount(ChangeType::StagedNew) } |
        tag!("staged_modified_count") => { |_| ParseExpression::ChangeCount(ChangeType::StagedModified) } |
        tag!("staged_deleted_count") => { |_| ParseExpression::ChangeCount(ChangeType::StagedDeleted) } |
        tag!("staged_renamed_count") => { |_| ParseExpression::ChangeCount(ChangeType::StagedRenamed) } |
        tag!("staged_typechange_count") => { |_| ParseExpression::ChangeCount(ChangeType::StagedTypechange) }
    ),
    char!('}')
));

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
    variable |
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
    fn single_item() {
        assert_eq!(variable(b"{branch}"), IResult::Done(&b""[..], ParseExpression::Branch));
        assert_eq!(variable(b"{commit_count}"), IResult::Done(&b""[..], ParseExpression::CommitCount));

        // Changes to the working tree
        assert_eq!(variable(b"{new_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::WTNew)));
        assert_eq!(variable(b"{modified_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::WTModified)));
        assert_eq!(variable(b"{deleted_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::WTDeleted)));
        assert_eq!(variable(b"{renamed_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::WTRenamed)));
        assert_eq!(variable(b"{typechange_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::WTTypechange)));

        // Staged changes
        assert_eq!(variable(b"{staged_new_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::StagedNew)));
        assert_eq!(variable(b"{staged_modified_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::StagedModified)));
        assert_eq!(variable(b"{staged_deleted_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::StagedDeleted)));
        assert_eq!(variable(b"{staged_renamed_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::StagedRenamed)));
        assert_eq!(variable(b"{staged_typechange_count}"), IResult::Done(&b""[..], ParseExpression::ChangeCount(ChangeType::StagedTypechange)));
    }

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
    fn one_of_each() {
        assert_eq!(
            expressions(b"!{branch}{commit_count}{new_count}{modified_count}{deleted_count}{renamed_count}{typechange_count}{staged_new_count}{staged_modified_count}{staged_deleted_count}{staged_renamed_count}{staged_typechange_count}"),
            IResult::Done(
                &b""[..],
                vec![
                    ParseExpression::Literal("!"),
                    ParseExpression::Branch,
                    ParseExpression::CommitCount,
                    ParseExpression::ChangeCount(ChangeType::WTNew),
                    ParseExpression::ChangeCount(ChangeType::WTModified),
                    ParseExpression::ChangeCount(ChangeType::WTDeleted),
                    ParseExpression::ChangeCount(ChangeType::WTRenamed),
                    ParseExpression::ChangeCount(ChangeType::WTTypechange),
                    ParseExpression::ChangeCount(ChangeType::StagedNew),
                    ParseExpression::ChangeCount(ChangeType::StagedModified),
                    ParseExpression::ChangeCount(ChangeType::StagedDeleted),
                    ParseExpression::ChangeCount(ChangeType::StagedRenamed),
                    ParseExpression::ChangeCount(ChangeType::StagedTypechange),
                ],
            )
        );
    }

    #[test]
    fn parse_single_param() {
        assert_eq!(single_param(b"(red)"), IResult::Done(&b""[..], &b"red"[..]));
    }
}
