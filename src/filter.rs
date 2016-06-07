use std::fs;

trait Filter {
    fn filter(&mut self, f: fn(Vec<fs::DirEntry>) -> Vec<fs::DirEntry>) -> Vec<fs::DirEntry>;
}

impl Filter for fs::ReadDir {
    fn filter(&mut self, f: fn(Vec<fs::DirEntry>) -> Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
        let mut files = Vec::new();

        for f in self {
            match f {
                Ok(f) => files.push(f),
                _ => (),
            }
        }

        f(files)
    }
}
