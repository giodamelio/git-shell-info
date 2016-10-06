use super::{ParseExpression, ChangeType};

// Convert a variable to a parse expression
named!(pub variable_parser<&[u8], ParseExpression>, delimited!(
    char!('{'),
    alt_complete!(
        tag!("branch") => { |_|
            ParseExpression::Branch
        } |
        tag!("commit_count") => { |_|
            ParseExpression::CommitCount
        } |

        // Changes in the working tree
        tag!("new_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::WTNew)
        } |
        tag!("modified_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::WTModified)
        } |
        tag!("deleted_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::WTDeleted)
        } |
        tag!("renamed_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::WTRenamed)
        } |
        tag!("typechange_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::WTTypechange)
        } |

        // Stashed changes
        tag!("staged_new_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::StagedNew)
        } |
        tag!("staged_modified_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::StagedModified)
        } |
        tag!("staged_deleted_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::StagedDeleted)
        } |
        tag!("staged_renamed_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::StagedRenamed)
        } |
        tag!("staged_typechange_count") => { |_|
            ParseExpression::ChangeCount(ChangeType::StagedTypechange)
        }
    ),
    char!('}')
));

#[cfg(test)]
mod tests {
    use nom::IResult;

    use parser::{ParseExpression, ChangeType};

    use super::variable_parser;

    #[test]
    fn single_item() {
        assert_eq!(
            variable_parser(b"{branch}"),
            IResult::Done(&b""[..], ParseExpression::Branch)
        );
        assert_eq!(
            variable_parser(b"{commit_count}"),
            IResult::Done(&b""[..], ParseExpression::CommitCount)
        );

        // Changes to the working tree
        assert_eq!(
            variable_parser(b"{new_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::WTNew)
            )
        );
        assert_eq!(
            variable_parser(b"{modified_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::WTModified)
            )
        );
        assert_eq!(
            variable_parser(b"{deleted_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::WTDeleted)
            )
        );
        assert_eq!(
            variable_parser(b"{renamed_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::WTRenamed)
            )
        );
        assert_eq!(
            variable_parser(b"{typechange_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::WTTypechange)
            )
        );

        // Staged changes
        assert_eq!(
            variable_parser(b"{staged_new_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::StagedNew)
            )
        );
        assert_eq!(
            variable_parser(b"{staged_modified_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::StagedModified)
            )
        );
        assert_eq!(
            variable_parser(b"{staged_deleted_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::StagedDeleted)
            )
        );
        assert_eq!(
            variable_parser(b"{staged_renamed_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::StagedRenamed)
            )
        );
        assert_eq!(
            variable_parser(b"{staged_typechange_count}"),
            IResult::Done(
                &b""[..],
                ParseExpression::ChangeCount(ChangeType::StagedTypechange)
            )
        );
    }
}
