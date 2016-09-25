use std::{path, error};

use git2;
use git2::{Repository, ErrorCode};

quick_error! {
    #[derive(Debug)]
    pub enum GitInfoError {
        GitError(err: git2::Error) {
            from()
            description("git error")
            display("Git2 error: {}", err)
            cause(err)
        }
    }
}

pub struct GitInfo {
    repo: Repository,
}

impl GitInfo {
    pub fn new(path: path::PathBuf) -> Result<GitInfo, GitInfoError> {
        let repo = try!(Repository::open(path));

        Ok(GitInfo {
            repo: repo,
        })
    }

    pub fn branch_current(&self) -> Result<String, GitInfoError> {
        let head = try!(self.repo.head());
        Ok(head.shorthand().unwrap().to_owned())
        // let head = match self.repo.head() {
        //     Ok(head) => Some(head),
        //     Err(ref e) if e.code() == ErrorCode::UnbornBranch ||
        //         e.code() == ErrorCode::NotFound => None,
        //     Err(e) => return Err(e),
        // };
        // let head = head.as_ref().and_then(|h| h.shorthand());
        // head
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
