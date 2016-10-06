extern crate git2;
#[macro_use] extern crate nom;

mod errors;
mod parser;

use std::path;

use git2::{Repository, Branch, BranchType};

use parser::{ParseExpression, ChangeType};

// #[derive(Debug)]
pub struct GitInfo {
    repo: Repository,
}

impl GitInfo {
    pub fn new(path: path::PathBuf) -> Result<GitInfo, errors::GitInfoError> {
        let repo = try!(Repository::open(path));

        Ok(GitInfo {
            repo: repo,
        })
    }

    // Render a template
    pub fn format(&self, template: &str) -> Result<String, errors::GitInfoError> {
        // Parse the template
        let parsed = try!(parser::parse(template));

        // Get the current branch

        // Render the template with git data
        Ok(parsed.iter()
           // Render the data from git
           .map(|parse_item| self.parse_item_to_string(parse_item))
           // Render any errors at empty strings
           .map(|item| match item {
               Ok(i) => i,
               Err(_) => String::from(""),
           })
           .collect::<Vec<_>>()
           .concat())
    }

    // Convert a ParseExpression varient into a String
    fn parse_item_to_string(&self, parse_item: &ParseExpression) -> Result<String, errors::GitInfoError> {
        match *parse_item {
            // A non-ParseExpression string literal to be passed through to the output intact
            ParseExpression::Literal(text) =>
                Ok::<String, errors::GitInfoError>(text.to_owned()),
            // Get the name of the current branch
            ParseExpression::Branch => {
                let branch = try!(self.branch_current());
                let name = try!(branch.name());
                match name {
                    Some(n) => Ok(n.to_owned()),
                    None => Ok(String::from("")),
                }
            },
            // Count how many commits there are on the current branch
            ParseExpression::CommitCount => {
                let mut revwalk = try!(self.repo.revwalk());
                try!(revwalk.push_head());
                Ok(revwalk.count().to_string())
            },
            // Working Tree/Staged changes
            ParseExpression::ChangeCount(ref change_type) => {
                let count = try!(self.status_count_filter(change_type));
                Ok(count.to_string())
            },
            // A simple terminal color
            ParseExpression::Color(ref color) => {
                Ok(format!("{}", color))
            },
        }
    }

    // Gets the current branch
    pub fn branch_current(&self) -> Result<Branch, errors::GitInfoError> {
        // Get a reference to the head
        let head = try!(self.repo.head());

        // Make sure head is pointing to a branch
        if !head.is_branch() {
            return Err(errors::GitInfoError::BranchError);
        };

        // Get the name of the branch
        let name = match head.shorthand() {
            Some(name) => name,
            None => return Err(errors::GitInfoError::BranchError),
        };

        // Get the branch
        Ok(try!(self.repo.find_branch(name, BranchType::Local)))
    }

    // Get the count of files matching a status type
    fn status_count_filter(&self, status_type: &ChangeType) -> Result<usize, errors::GitInfoError> {
        let statuses = try!(self.repo.statuses(None));
        let modified_count = statuses.iter()
            .map(|status_entry| status_entry.status())
            .filter(|status| status.contains(match *status_type {
                // Working tree changes
                ChangeType::WTNew => git2::STATUS_WT_NEW,
                ChangeType::WTModified => git2::STATUS_WT_MODIFIED,
                ChangeType::WTDeleted => git2::STATUS_WT_DELETED,
                ChangeType::WTRenamed => git2::STATUS_WT_RENAMED,
                ChangeType::WTTypechange => git2::STATUS_WT_TYPECHANGE,
                // Staged changes
                ChangeType::StagedNew => git2::STATUS_INDEX_NEW,
                ChangeType::StagedModified => git2::STATUS_INDEX_MODIFIED,
                ChangeType::StagedDeleted => git2::STATUS_INDEX_DELETED,
                ChangeType::StagedRenamed => git2::STATUS_INDEX_RENAMED,
                ChangeType::StagedTypechange => git2::STATUS_INDEX_TYPECHANGE,
            }))
            .count();

        Ok(modified_count)
    }

    // Gets count of commits the current branch is ahead of its upstream
    fn branch_upstream_ahead(&self) -> Result<usize, errors::GitInfoError> {
        Ok(10)
    }
}
