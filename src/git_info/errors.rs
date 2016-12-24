use std::fmt;
use std::error::Error;

use git2;
use edo;

#[derive(Debug)]
pub enum GitInfoError {
    LibGitError(git2::Error),
    EdoError(edo::error::EdoError),
    BranchError,
}

impl fmt::Display for GitInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitInfoError::LibGitError(ref err) => err.fmt(f),
            GitInfoError::EdoError(ref err) => write!(f, "Edo error: {:?}", err),
            GitInfoError::BranchError => write!(f, "Branch error"),
        }
    }
}

impl Error for GitInfoError {
    fn description(&self) -> &str {
        match *self {
            GitInfoError::LibGitError(ref err) => err.description(),
            GitInfoError::EdoError(ref err) => err.description(),
            GitInfoError::BranchError => "Branch error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GitInfoError::LibGitError(ref err) => Some(err),
            GitInfoError::EdoError(ref err) => Some(err),
            GitInfoError::BranchError => None,
        }
    }
}

impl From<git2::Error> for GitInfoError {
    fn from(err: git2::Error) -> GitInfoError {
        GitInfoError::LibGitError(err)
    }
}

impl From<edo::error::EdoError> for GitInfoError {
    fn from(err: edo::error::EdoError) -> GitInfoError {
        GitInfoError::EdoError(err)
    }
}
