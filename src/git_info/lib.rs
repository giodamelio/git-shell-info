extern crate git2;
extern crate edo;

mod errors;
mod types;

use std::path;

use git2::{Repository, Branch, BranchType};
use edo::Edo;

use types::{ChangeType};

// #[derive(Debug)]
pub struct GitInfo {
    repo: Repository,
}

impl GitInfo {
    pub fn new<'a>(path: path::PathBuf) -> Result<GitInfo, errors::GitInfoError> {
        let repo = try!(Repository::open(path));

        Ok(GitInfo {
            repo: repo,
        })
    }

    // Render a template
    pub fn format<'a>(&'a self, template: &'a str) -> Result<String, errors::GitInfoError> {
        // Setup the template with Edo
        let mut template = try!(Edo::new(template));

        // Handle color statements
        // TODO: test the bold colors
        template.register_handler("color", |args| {
            if args.len() == 1 {
                let color_code = (match args[0] {
                    "reset" => "0",
                    "black" => "0;30",
                    "red" => "0;31",
                    "green" => "0;32",
                    "yellow" => "0;33",
                    "blue" => "0;34",
                    "magenta" => "0;35",
                    "cyan" => "0;36",
                    "white" => "0;37",
                    "bold_black" => "1;30",
                    "bold_red" => "1;31",
                    "bold_green" => "1;32",
                    "bold_yellow" => "1;33",
                    "bold_blue" => "1;34",
                    "bold_magenta" => "1;35",
                    "bold_cyan" => "1;36",
                    "bold_white" => "1;37",
                    _ => "0",
                }).to_string();

                format!("\x1b[{}m", color_code)
            } else {
                "".to_string()
            }
        });

        // Handle true color statements
        template.register_handler("rgb", |args| {
            if args.len() == 3 {
                format!("\x1b[38;2;{};{};{}m", args[0], args[1], args[2])
            } else {
                "".to_string()
            }
        });

        Ok(template.render())
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
