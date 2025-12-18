use anyhow::Result;
use git2::{Repository, Sort, Status, StatusOptions};
use std::path::PathBuf;

pub struct GitInfo {
    pub branches: Vec<String>,
    pub commits: Vec<String>,
    pub staged: Vec<PathBuf>,
}

pub fn get_repo_info() -> Result<GitInfo> {
    let repo = Repository::discover(".")?;

    //Get Branches
    let mut branches = Vec::new();
    let git_branches = repo.branches(None)?;
    for b in git_branches {
        let (branch, _) = b?;
        if let Some(name) = branch.name()? {
            let prefix = if branch.is_head() { "* " } else { "  " };
            branches.push(format!("{}{}", prefix, name));
        }
    }

    //Get Commits
    let mut commits = Vec::new();
    if let Ok(head) = repo.head() {
        if let Ok(commit) = head.peel_to_commit() {
            let mut revwalk = repo.revwalk()?;
            revwalk.push(commit.id())?;
            revwalk.set_sorting(Sort::TIME)?;

            for id in revwalk.take(20) {
                if let Ok(oid) = id {
                    if let Ok(c) = repo.find_commit(oid) {
                        let msg = c.summary().unwrap_or("No message");
                        let short_id = &oid.to_string()[..7];
                        commits.push(format!("{} - {}", short_id, msg));
                    }
                }
            }
        }
    }

    //Get Staged Files
    let mut staged = Vec::new();
    let mut opts = StatusOptions::new();
    opts.include_untracked(false); // Only look at staged/tracked

    let statuses = repo.statuses(Some(&mut opts))?;
    for entry in statuses.iter() {
        let status = entry.status();
        if status.contains(Status::INDEX_NEW)
            || status.contains(Status::INDEX_MODIFIED)
            || status.contains(Status::INDEX_DELETED)
        {
            if let Some(path) = entry.path() {
                staged.push(PathBuf::from(path));
            }
        }
    }

    Ok(GitInfo {
        branches,
        commits,
        staged,
    })
}
