use std::path;

use git2;
use git2::Repository;

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

#[derive(RustcEncodable)]
pub struct GitInfo {
    branch_current: String,
}

impl GitInfo {
    pub fn new(path: path::PathBuf) -> Result<GitInfo, GitInfoError> {
        let repo = try!(Repository::open(path));

        Ok(GitInfo {
            branch_current: try!(GitInfo::branch_current(repo)),
        })
    }

    pub fn branch_current(repo: git2::Repository) -> Result<String, GitInfoError> {
        let head = try!(repo.head());
        match head.shorthand() {
            Some(shorthand) => Ok(shorthand.to_owned()),
            None => Ok(String::from("NO BRANCH")),
        }
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
