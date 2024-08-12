// Import necessary crates and modules
extern crate git2;
extern crate walkdir;
use git2::Repository;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

// Define DirStats struct
pub struct DirStats {
    pub file_count: usize,
    pub line_count: usize,
    pub commit_count: usize,
}

// Implement methods for DirStats
impl DirStats {
    // Constructor for DirStats
    pub fn new() -> DirStats {
        DirStats {
            file_count: 0,
            line_count: 0,
            commit_count: 0,
        }
    }

    // Method to gather stats for a given directory
    pub fn gather_stats(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Read .gitignore file and store ignored patterns
        let gitignore_path = path.join(".gitignore");
        let mut ignored_patterns = Vec::new();
        if gitignore_path.exists() {
            let file = File::open(&gitignore_path)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if !line.starts_with('#') && !line.trim().is_empty() {
                    ignored_patterns.push(line);
                }
            }
        }

        // Iterate over all entries in the directory
        for entry in WalkDir::new(path) {
            let entry = entry?;
            // If the entry is a file, increment file count and line count
            if entry.file_type().is_file() {
                let is_ignored = ignored_patterns.iter().any(|pattern| {
                    let pattern = glob::Pattern::new(pattern).expect("Invalid glob pattern");
                    pattern.matches_path(entry.path().strip_prefix(path).unwrap())
                });
                if !is_ignored {
                    self.file_count += 1;
                    self.line_count += count_lines(&entry.path())?;
                }
            }
        }

        // Try to open the directory as a git repository
        let repo = Repository::discover(path);
        // If successful, count the number of commits
        if let Ok(r) = repo {
            let mut revwalk = r.revwalk().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let first_commit_hash = find_first_commit_hash(&r)?;
            revwalk.push_range(&format!("{}..HEAD", first_commit_hash))?;
            revwalk.set_sorting(git2::Sort::TIME)?;
            for _ in revwalk {
                self.commit_count += 1;
            }

            // Iterate over the submodules and recursively gather stats
            for submodule in r.submodules()? {
                let submodule_path = path.join(submodule.path());
                self.gather_stats(&submodule_path)?;
            }
        }

        Ok(())
    }
}

// Function to count the number of lines in a file
fn count_lines(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count)
}
// Function to find the hash of the first commit
fn find_first_commit_hash(repo: &Repository) -> Result<String, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::NONE)?;
    let oid = revwalk.last();
    if let Some(Ok(hash)) = oid {
        return Ok(hash.to_string())
    };
    return Ok("No commits".to_string())
}
