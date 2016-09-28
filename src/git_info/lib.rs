extern crate git2;
#[macro_use] extern crate nom;

mod errors;
mod parser;

use std::path;

use git2::{Repository, Branch, BranchType};

use parser::ParseItem;

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

    pub fn format(&self, template: &str) -> Result<String, errors::GitInfoError> {
        // Parse the template
        let parsed = try!(parser::parse(template));

        // Get the current branch

        // Render the template with git data
        Ok(parsed.iter()
           // Render the data from git
           .map(|item| {
               match *item {
                   ParseItem::Literal(text) =>
                       Ok::<String, errors::GitInfoError>(text.to_owned()),
                   ParseItem::Branch => {
                       let branch = try!(self.branch_current());
                       let name = try!(branch.name());
                       match name {
                           Some(n) => Ok(n.to_owned()),
                           None => Ok(String::from("")),
                       }
                   },
                   ParseItem::CommitCount => {
                       let mut revwalk = try!(self.repo.revwalk());
                       try!(revwalk.push_head());
                       Ok(revwalk.count().to_string())
                   },
                   ParseItem::Modified => {
                       Ok(String::from("test"))
                   },
               }
           })
           // Render any errors at empty strings
           .map(|item| match item {
               Ok(i) => i,
               Err(_) => String::from(""),
           })
           .collect::<Vec<_>>()
           .concat())
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

    // Gets count of commits the current branch is ahead of its upstream
    fn branch_upstream_ahead(&self) -> Result<usize, errors::GitInfoError> {
        Ok(10)
    }
}

// fn status() {
//     for status_entry in repo.statuses(None).unwrap().iter() {
//         let status = status_entry.status();
//         let path = status_entry.path().unwrap();
//         let status_text = match status {
//             s if s.contains(git2::STATUS_WT_MODIFIED) => "modified",
//             s if s.contains(git2::STATUS_WT_DELETED) => "deleted",
//             s if s.contains(git2::STATUS_WT_RENAMED) => "renamed",
//             s if s.contains(git2::STATUS_WT_NEW) => "new",
//             s if s.contains(git2::STATUS_WT_TYPECHANGE) => "typechange",
//             _ => continue,
//         };
//         println!("{}: {}", status_text, path);
//     }
// }
