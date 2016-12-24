extern crate git2;
extern crate edo;

mod errors;
mod types;
mod handlers;

use std::path;

use git2::{Repository, Branch, BranchType};
use edo::Edo;

use types::{ChangeType};
use handlers::{color_handler, rgb_handler};

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
        template.register_handler("color", color_handler);

        // Handle true color statements
        template.register_handler("rgb", rgb_handler);

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
