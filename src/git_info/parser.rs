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
pub enum TerminalColor {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BoldBlack,
    BoldRed,
    BoldGreen,
    BoldYellow,
    BoldBlue,
    BoldMagenta,
    BoldCyan,
    BoldWhite,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseItem<'a> {
    Literal(&'a str),
    Branch,
    CommitCount,
    ChangeCount(ChangeType),
    Color(TerminalColor),
}

// A single item
named!(item<&[u8], ParseItem>, delimited!(
    char!('{'),
    alt_complete!(
        tag!("branch") => { |_| ParseItem::Branch } |
        tag!("commit_count") => { |_| ParseItem::CommitCount } |

        // Changes in the working tree
        tag!("new_count") => { |_| ParseItem::ChangeCount(ChangeType::WTNew) } |
        tag!("modified_count") => { |_| ParseItem::ChangeCount(ChangeType::WTModified) } |
        tag!("deleted_count") => { |_| ParseItem::ChangeCount(ChangeType::WTDeleted) } |
        tag!("renamed_count") => { |_| ParseItem::ChangeCount(ChangeType::WTRenamed) } |
        tag!("typechange_count") => { |_| ParseItem::ChangeCount(ChangeType::WTTypechange) } |

        // Stashed changes
        tag!("staged_new_count") => { |_| ParseItem::ChangeCount(ChangeType::StagedNew) } |
        tag!("staged_modified_count") => { |_| ParseItem::ChangeCount(ChangeType::StagedModified) } |
        tag!("staged_deleted_count") => { |_| ParseItem::ChangeCount(ChangeType::StagedDeleted) } |
        tag!("staged_renamed_count") => { |_| ParseItem::ChangeCount(ChangeType::StagedRenamed) } |
        tag!("staged_typechange_count") => { |_| ParseItem::ChangeCount(ChangeType::StagedTypechange) }
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
named!(function<&[u8], ParseItem>, delimited!(
    char!('{'),
    alt!(
        tuple!(
            tag!("color"),
            single_param
        ) => { |params: (&[u8], &[u8])| ParseItem::Color(match params.1 {

            b"black" => TerminalColor::Black,
            b"red" => TerminalColor::Red,
            b"green" => TerminalColor::Green,
            b"yellow" => TerminalColor::Yellow,
            b"blue" => TerminalColor::Blue,
            b"magenta" => TerminalColor::Magenta,
            b"cyan" => TerminalColor::Cyan,
            b"white" => TerminalColor::White,
            b"bold_black" => TerminalColor::BoldBlack,
            b"bold_red" => TerminalColor::BoldRed,
            b"bold_green" => TerminalColor::BoldGreen,
            b"bold_yellow" => TerminalColor::BoldYellow,
            b"bold_blue" => TerminalColor::BoldBlue,
            b"bold_magenta" => TerminalColor::BoldMagenta,
            b"bold_cyan" => TerminalColor::BoldCyan,
            b"bold_white" => TerminalColor::BoldWhite,
            &_ => TerminalColor::Reset,
        }) }
    ),
    char!('}')
));

// One or more items seperated some string literals
named!(items<&[u8], Vec<ParseItem> >, many0!(alt!(
    function |
    item |
    map!(
        map_res!(
            take_while!(brace_or_eol),
            str::from_utf8
        ),
        ParseItem::Literal
    )
)));

pub fn parse(template: &str) -> Result<Vec<ParseItem>, GitInfoError> {
    match items(template.as_bytes()) {
        IResult::Done(_, result) => Ok(result),
        IResult::Error(_) | IResult::Incomplete(_) => Err(GitInfoError::ParseError),
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::{ParseItem, ChangeType, TerminalColor, item, items, single_param, function};

    #[test]
    fn single_item() {
        assert_eq!(item(b"{branch}"), IResult::Done(&b""[..], ParseItem::Branch));
        assert_eq!(item(b"{commit_count}"), IResult::Done(&b""[..], ParseItem::CommitCount));

        // Changes to the working tree
        assert_eq!(item(b"{new_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::WTNew)));
        assert_eq!(item(b"{modified_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::WTModified)));
        assert_eq!(item(b"{deleted_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::WTDeleted)));
        assert_eq!(item(b"{renamed_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::WTRenamed)));
        assert_eq!(item(b"{typechange_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::WTTypechange)));

        // Staged changes
        assert_eq!(item(b"{staged_new_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::StagedNew)));
        assert_eq!(item(b"{staged_modified_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::StagedModified)));
        assert_eq!(item(b"{staged_deleted_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::StagedDeleted)));
        assert_eq!(item(b"{staged_renamed_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::StagedRenamed)));
        assert_eq!(item(b"{staged_typechange_count}"), IResult::Done(&b""[..], ParseItem::ChangeCount(ChangeType::StagedTypechange)));
    }

    #[test]
    fn single_function() {
        assert_eq!(function(b"{color(red)}"), IResult::Done(&b""[..], ParseItem::Color(TerminalColor::Red)));
    }

    #[test]
    fn three_items() {
        assert_eq!(
            items(b"{branch}{commit_count}{branch}"),
            IResult::Done(
                &b""[..],
                vec![ParseItem::Branch, ParseItem::CommitCount, ParseItem::Branch],
            )
        );
    }

    #[test]
    fn three_items_with_literals() {
        assert_eq!(
            items(b"{branch}|{branch}|{commit_count}"),
            IResult::Done(
                &b""[..],
                vec![
                    ParseItem::Branch,
                    ParseItem::Literal("|"),
                    ParseItem::Branch,
                    ParseItem::Literal("|"),
                    ParseItem::CommitCount,
                ],
            )
        );
    }

    #[test]
    fn items_with_literals_at_end() {
        assert_eq!(
            items(b"{branch}YAY"),
            IResult::Done(
                &b""[..],
                vec![ParseItem::Branch, ParseItem::Literal("YAY")],
            )
        );
    }

    #[test]
    fn one_of_each() {
        assert_eq!(
            items(b"!{branch}{commit_count}{new_count}{modified_count}{deleted_count}{renamed_count}{typechange_count}{staged_new_count}{staged_modified_count}{staged_deleted_count}{staged_renamed_count}{staged_typechange_count}"),
            IResult::Done(
                &b""[..],
                vec![
                    ParseItem::Literal("!"),
                    ParseItem::Branch,
                    ParseItem::CommitCount,
                    ParseItem::ChangeCount(ChangeType::WTNew),
                    ParseItem::ChangeCount(ChangeType::WTModified),
                    ParseItem::ChangeCount(ChangeType::WTDeleted),
                    ParseItem::ChangeCount(ChangeType::WTRenamed),
                    ParseItem::ChangeCount(ChangeType::WTTypechange),
                    ParseItem::ChangeCount(ChangeType::StagedNew),
                    ParseItem::ChangeCount(ChangeType::StagedModified),
                    ParseItem::ChangeCount(ChangeType::StagedDeleted),
                    ParseItem::ChangeCount(ChangeType::StagedRenamed),
                    ParseItem::ChangeCount(ChangeType::StagedTypechange),
                ],
            )
        );
    }

    #[test]
    fn parse_single_param() {
        assert_eq!(single_param(b"(red)"), IResult::Done(&b""[..], &b"red"[..]));
    }
}
