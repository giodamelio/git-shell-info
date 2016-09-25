extern crate git2;

use std::env;

use git2::Repository;

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Error: {}", e),
    };

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Error: {}", e),
    };

    for status_entry in repo.statuses(None).unwrap().iter() {
        let status = status_entry.status();
        let path = status_entry.path().unwrap();
        let status_text = match status {
            s if s.contains(git2::STATUS_WT_MODIFIED) => "modified",
            s if s.contains(git2::STATUS_WT_DELETED) => "deleted",
            s if s.contains(git2::STATUS_WT_RENAMED) => "renamed",
            s if s.contains(git2::STATUS_WT_NEW) => "new",
            s if s.contains(git2::STATUS_WT_TYPECHANGE) => "typechange",
            _ => continue,
        };
        println!("{}: {}", status_text, path);
    }
}
