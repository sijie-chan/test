use git2::{Commit, Repository};
pub struct GitService {
    repo: Repository,
}

impl GitService {
    pub fn new(repo_path: &str) -> Self {
        GitService {
            repo: Repository::init(dir_path),
        }
    }
}
