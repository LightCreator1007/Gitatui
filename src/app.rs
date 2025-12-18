use crate::git_logic;
use std::path::PathBuf;

// Color theme
pub mod theme {
    use ratatui::style::Color;

    pub const BG: Color = Color::Rgb(26, 27, 38);
    pub const FG: Color = Color::Rgb(169, 177, 214);
    pub const SELECTION: Color = Color::Rgb(122, 162, 247);
    pub const COMMENT: Color = Color::Rgb(86, 95, 137);
    pub const GREEN: Color = Color::Rgb(158, 206, 106);
    pub const ORANGE: Color = Color::Rgb(224, 175, 104);
    pub const RED: Color = Color::Rgb(247, 118, 142);
    pub const WHITE: Color = Color::Rgb(255, 255, 255);
}

#[derive(Debug, Clone, PartialEq)]
pub enum Focus {
    Branches,
    Commits,
    Staged,
}

pub struct App {
    pub should_quit: bool,
    pub current_focus: Focus,
    pub branches: Vec<String>,
    pub commits: Vec<String>,
    pub staged_files: Vec<PathBuf>,
    pub branch_idx: usize,
    pub commit_idx: usize,
    pub staged_idx: usize,
}

impl App {
    pub fn new() -> Self {
        let (branches, commits, staged_files) = match git_logic::get_repo_info() {
            Ok(info) => (info.branches, info.commits, info.staged),
            Err(_) => (
                vec![
                    "* main".to_string(),
                    "  dev".to_string(),
                    "  feature/ui".to_string(),
                ],
                vec![
                    "a1b2c3d - Mock Commit 1".to_string(),
                    "e5f6g7h - Mock Commit 2".to_string(),
                ],
                vec![PathBuf::from("src/main.rs"), PathBuf::from("Cargo.toml")],
            ),
        };
        Self {
            should_quit: false,
            current_focus: Focus::Branches,
            branches,
            commits,
            staged_files,
            branch_idx: 0,
            commit_idx: 0,
            staged_idx: 0,
        }
    }

    fn update_selection(&mut self) {
        // Only update if we are focusing on branches
        if self.branches.is_empty() {
            return;
        }

        // 1. Get the selected branch string (e.g., "* main" or "  dev")
        let raw_name = &self.branches[self.branch_idx];

        // 2. Clean the name (remove "* " or "  ")
        let clean_name = raw_name.trim_start_matches(|c| c == '*' || c == ' ').trim();

        // 3. Fetch commits for this specific branch
        if let Ok(new_commits) = git_logic::get_commits_from_name(clean_name) {
            self.commits = new_commits;
            self.commit_idx = 0; // Reset scroll to top
        }
    }

    pub fn on_up(&mut self) {
        match self.current_focus {
            Focus::Branches => {
                self.branch_idx = self.branch_idx.saturating_sub(1);
                self.update_selection(); // <--- Trigger Update
            }
            Focus::Commits => self.commit_idx = self.commit_idx.saturating_sub(1),
            Focus::Staged => self.staged_idx = self.staged_idx.saturating_sub(1),
        }
    }

    pub fn on_down(&mut self) {
        match self.current_focus {
            Focus::Branches => {
                if !self.branches.is_empty() && self.branch_idx < self.branches.len() - 1 {
                    self.branch_idx += 1;
                    self.update_selection(); // <--- Trigger Update
                }
            }
            Focus::Commits => {
                if !self.commits.is_empty() && self.commit_idx < self.commits.len() - 1 {
                    self.commit_idx += 1;
                }
            }
            Focus::Staged => {
                if !self.staged_files.is_empty() && self.staged_idx < self.staged_files.len() - 1 {
                    self.staged_idx += 1;
                }
            }
        }
    }

    pub fn on_right(&mut self) {
        self.current_focus = match self.current_focus {
            Focus::Branches => Focus::Commits,
            Focus::Commits => Focus::Staged,
            Focus::Staged => Focus::Branches,
        };
    }

    pub fn on_left(&mut self) {
        self.current_focus = match self.current_focus {
            Focus::Branches => Focus::Staged,
            Focus::Commits => Focus::Branches,
            Focus::Staged => Focus::Commits,
        };
    }
}
