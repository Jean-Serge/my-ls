use std::fs;

trait Sorter {
    fn sort(&mut self, f: fn(Vec<fs::DirEntry>) -> Vec<fs::DirEntry>) -> Vec<fs::DirEntry>;
}

impl Sorter for fs::ReadDir {
    fn sort(&mut self, f: fn(Vec<fs::DirEntry>) -> Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
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
