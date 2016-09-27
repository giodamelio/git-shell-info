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
    Remote,
}

// A single item
named!(item<&[u8], ParseItem>, delimited!(
    char!('{'),
    alt!(
        tag!("branch") => { |_| ParseItem::Branch } |
        tag!("remote") => { |_| ParseItem::Remote }
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
        assert_eq!(item(b"{remote}"), IResult::Done(&b""[..], ParseItem::Remote));
    }

    #[test]
    fn three_items() {
        assert_eq!(
            items(b"{branch}{remote}{branch}"),
            IResult::Done(
                &b""[..],
                vec![ParseItem::Branch, ParseItem::Remote, ParseItem::Branch],
            )
        );
    }

    #[test]
    fn three_items_with_literals() {
        assert_eq!(
            items(b"{branch}|{remote}|{branch}"),
            IResult::Done(
                &b""[..],
                vec![
                    ParseItem::Branch,
                    ParseItem::Literal("|"),
                    ParseItem::Remote,
                    ParseItem::Literal("|"),
                    ParseItem::Branch
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
