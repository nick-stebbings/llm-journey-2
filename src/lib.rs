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
        // Iterate over all entries in the directory
        for entry in WalkDir::new(path) {
            let entry = entry?;
            // If the entry is a file, increment file count and line count
            if entry.file_type().is_file() {
                self.file_count += 1;
                self.line_count += count_lines(&entry.path())?;
            }
        }

        // Try to open the directory as a git repository
        let repo = Repository::discover(path);
        // If successful, count the number of commits
        if let Ok(r) = repo {
            let mut revwalk = r.revwalk().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            revwalk.push_range("HEAD~10000..HEAD")?;
            revwalk.set_sorting(git2::Sort::TIME)?;
            self.commit_count += revwalk.count() as usize;

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
