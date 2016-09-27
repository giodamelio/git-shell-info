use std::fmt;
use std::error::Error;

use git2;

#[derive(Debug)]
pub enum GitInfoError {
    LibGitError(git2::Error),
    ParseError,
    BranchError,
}

impl fmt::Display for GitInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitInfoError::LibGitError(ref err) => err.fmt(f),
            GitInfoError::ParseError => write!(f, "Parse error"),
            GitInfoError::BranchError => write!(f, "Branch error"),
        }
    }
}

impl Error for GitInfoError {
    fn description(&self) -> &str {
        match *self {
            GitInfoError::LibGitError(ref err) => err.description(),
            GitInfoError::ParseError => "Parse error",
            GitInfoError::BranchError => "Branch error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GitInfoError::LibGitError(ref err) => Some(err),
            GitInfoError::ParseError => None,
            GitInfoError::BranchError => None,
        }
    }
}

impl From<git2::Error> for GitInfoError {
    fn from(err: git2::Error) -> GitInfoError {
        GitInfoError::LibGitError(err)
    }
}
