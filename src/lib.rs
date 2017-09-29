extern crate git2;

use git2::Repository;

pub fn open_repo() {
    let repo = match Repository::open(env!("CARGO_MANIFEST_DIR")) {
        Ok(repo) => repo,
        Err(e) => panic!("falied to open repo: {}", e),
    };
    println!("Opened repo: {:#?}", repo.path());
}

#[cfg(test)]
mod tests {
    use super::open_repo;

    #[test]
    fn opens_a_repo() {
        open_repo();
        assert_eq!(2 + 2, 4);
    }
}
