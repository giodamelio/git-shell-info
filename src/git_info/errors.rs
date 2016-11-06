use std::fmt;
use std::error::Error;

use git2;
use nom;

#[derive(Debug)]
pub enum GitInfoError<'a> {
    LibGitError(git2::Error),
    ParseError(nom::Err<&'a [u8]>),
    BranchError,
}

impl<'a> fmt::Display for GitInfoError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitInfoError::LibGitError(ref err) => err.fmt(f),
            GitInfoError::ParseError(ref err) => write!(f, "Parse error: {:?}", err),
            GitInfoError::BranchError => write!(f, "Branch error"),
        }
    }
}

impl<'a> Error for GitInfoError<'a> {
    fn description(&self) -> &str {
        match *self {
            GitInfoError::LibGitError(ref err) => err.description(),
            GitInfoError::ParseError(_) => "Parse error",
            GitInfoError::BranchError => "Branch error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GitInfoError::LibGitError(ref err) => Some(err),
            GitInfoError::ParseError(_) => None,
            GitInfoError::BranchError => None,
        }
    }
}

impl<'a> From<git2::Error> for GitInfoError<'a> {
    fn from(err: git2::Error) -> GitInfoError<'a> {
        GitInfoError::LibGitError(err)
    }
}
