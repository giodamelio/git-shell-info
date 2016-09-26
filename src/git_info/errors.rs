use git2;

quick_error! {
    #[derive(Debug)]
    pub enum GitInfoError {
        LibGitError(err: git2::Error) {
            from()
            description("git error")
            display("Git2 error: {}", err)
            cause(err)
        }
        BranchError(err: &'static str) {
            from()
            description("branch error")
            display("Branch error: {}", err)
        }
    }
}
