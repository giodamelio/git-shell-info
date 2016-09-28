use std::str;

use nom::IResult;

use super::errors::GitInfoError;

// Go until there is a {
fn brace_or_eol(char: u8) -> bool {
    char != b'{'
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseItem<'a> {
    Literal(&'a str),
    Branch,
    CommitCount,
}

// A single item
named!(item<&[u8], ParseItem>, delimited!(
    char!('{'),
    alt!(
        tag!("branch") => { |_| ParseItem::Branch } |
        tag!("commit_count") => { |_| ParseItem::CommitCount }
    ),
    char!('}')
));

// One or more items seperated some string literals
named!(items<&[u8], Vec<ParseItem> >, many0!(alt!(
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

    use super::{ParseItem, item, items};

    #[test]
    fn single_item() {
        assert_eq!(item(b"{branch}"), IResult::Done(&b""[..], ParseItem::Branch));
        assert_eq!(item(b"{commit_count}"), IResult::Done(&b""[..], ParseItem::CommitCount));
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
}
