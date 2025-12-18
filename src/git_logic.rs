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

    // 1. Get Branches
    let mut branches = Vec::new();
    let git_branches = repo.branches(None)?;
    for b in git_branches {
        let (branch, _) = b?;
        if let Some(name) = branch.name()? {
            let prefix = if branch.is_head() { "* " } else { "  " };
            branches.push(format!("{}{}", prefix, name));
        }
    }

    // 2. Initial Staged Files
    let staged = get_staged_files(&repo)?;

    // 3. Initial Commits (HEAD)
    let commits = get_commits_for_branch(&repo, "HEAD")?;

    Ok(GitInfo {
        branches,
        commits,
        staged,
    })
}

// --- NEW HELPER FUNCTIONS ---

pub fn get_staged_files(repo: &Repository) -> Result<Vec<PathBuf>> {
    let mut staged = Vec::new();
    let mut opts = StatusOptions::new();
    opts.include_untracked(false);

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
    Ok(staged)
}

// Now accepts a specific branch name (e.g., "main", "dev", "HEAD")
pub fn get_commits_from_name(branch_name: &str) -> Result<Vec<String>> {
    let repo = Repository::discover(".")?;
    get_commits_for_branch(&repo, branch_name)
}

fn get_commits_for_branch(repo: &Repository, branch_ref: &str) -> Result<Vec<String>> {
    let mut commits = Vec::new();

    // Resolve string name to object (Commit)
    let obj = repo.revparse_single(branch_ref)?;
    let commit = obj.peel_to_commit()?;

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
    Ok(commits)
}
