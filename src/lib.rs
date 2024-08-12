extern crate git2;
extern crate walkdir;

use git2::Repository;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct DirStats {
    pub file_count: usize,
    pub line_count: usize,
    pub commit_count: usize,
}

impl DirStats {
    pub fn new() -> DirStats {
        DirStats {
            file_count: 0,
            line_count: 0,
            commit_count: 0,
        }
    }

    pub fn gather_stats(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                self.file_count += 1;
                self.line_count += count_lines(&entry.path())?;
            }
        }

        let repo = Repository::discover(path);
        if let Ok(r) = repo {
            let revwalk = r
                .revwalk()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            self.commit_count = revwalk.count() as usize;
        }

        Ok(())
    }
}

fn count_lines(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count)
}
