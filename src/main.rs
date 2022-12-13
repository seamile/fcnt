pub mod color;

use clap::Parser;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::os::macos::fs::MetadataExt;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[command(name = "fcnt")]
#[command(version = "0.1.0")]
#[command(about = "Count the total number of files in given directories.")]
struct CmdLineArgs {
    /// the directories (default: ./)
    directories: Vec<String>,

    /// count all regular and hidden files.
    #[arg(short = 'a')]
    all_files: bool,

    /// count the number of directories.
    #[arg(short = 'd')]
    count_dirs: bool,

    /// count the total size of files.
    #[arg(short = 's')]
    count_size: bool,
}

impl CmdLineArgs {
    fn get_directories(&self) -> Vec<PathBuf> {
        let mut directories: Vec<PathBuf> = Vec::new();
        if self.directories.is_empty() {
            directories.push(env::current_dir().unwrap());
        } else {
            for dir in self.directories.iter().map(|p| PathBuf::from(p)) {
                if dir.is_dir() {
                    directories.push(dir);
                } else {
                    let msg = format!("fcnt: {:?} is not directory.", dir);
                    println!("{}", color::err(&msg));
                }
            }
            if directories.is_empty() {
                println!("{}", color::err(&"fcnt: non directories."));
                exit(1);
            }
        }
        return directories;
    }
}

struct Counter {
    pub n_files: u64,
    pub n_dirs: u64,
    pub size: u64,
    inodes: HashSet<u64>,
}

impl Counter {
    const SZ_UNIT: [&str; 7] = ["B", "K", "M", "G", "T", "P", "E"];

    pub fn new() -> Counter {
        return Counter {
            n_files: 0,
            n_dirs: 0,
            size: 0,
            inodes: HashSet::new(),
        };
    }

    // the total block size used by a file
    fn file_size(metadata: &fs::Metadata) -> u64 {
        let sz = metadata.st_size() as f64;
        let blksz = metadata.st_blksize() as f64;
        return (blksz * (sz / blksz).ceil()) as u64;
    }

    // Make "size" more readable
    fn readable_size(&self) -> String {
        let mut sz = self.size as f64;
        let mut str_sz = String::new();

        for unit in Counter::SZ_UNIT {
            if sz >= 1024.0 {
                sz = sz / 1024.0;
            } else {
                if sz.fract() < 0.05 {
                    str_sz = format!("{:.0}{}", sz, unit);
                } else {
                    str_sz = format!("{:.1}{}", sz, unit);
                }
                break;
            }
        }
        return str_sz;
    }

    // count a entry
    pub fn count(&mut self, entry: fs::DirEntry) {
        let path = entry.path();

        if path.is_symlink() {
            // ignore the size of symbolic link
            self.n_files += 1;
        } else if path.is_file() {
            let Ok(meta) = entry.metadata() else {return;};
            if self.inodes.insert(meta.st_ino()) {
                // count the number and the size of regular file
                self.n_files += 1;
                self.size += Counter::file_size(&meta);
            } else {
                // ignore the size of hard link
                self.n_files += 1;
            }
        } else if path.is_dir() {
            self.n_dirs += 1;
        }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "files: {}  dirs: {}  size: {}",
            self.n_files,
            self.n_dirs,
            self.readable_size()
        );
    }
}

// walk all the files in dir
struct Walker {
    dirs: Vec<PathBuf>,
    reader: fs::ReadDir,
}

impl Walker {
    pub fn new(dirpath: PathBuf) -> Walker {
        if dirpath.is_dir() {
            match fs::read_dir(&dirpath) {
                Ok(rd) => Walker {
                    dirs: vec![dirpath],
                    reader: rd,
                },
                Err(msg) => panic!("error: {}", msg),
            }
        } else {
            panic!("error: the {:?} is not a directory.", dirpath);
        }
    }
}

impl Iterator for Walker {
    type Item = fs::DirEntry;

    fn next(&mut self) -> Option<fs::DirEntry> {
        if let Some(entry) = self.reader.next() {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_dir() {
                        self.dirs.push(path);
                    };
                    return Some(entry);
                }
                Err(msg) => panic!("error: {}", msg),
            }
        } else if let Some(path) = self.dirs.pop() {
            self.reader = fs::read_dir(path).unwrap();
            return self.next();
        } else {
            return None;
        };
    }
}

fn main() {
    // parse cmd-line args and get directories
    let args = CmdLineArgs::parse();
    let directories = args.get_directories();

    // walk all files in the directories
    for dir in directories {
        let mut counter = Counter::new();
        for entry in Walker::new(dir) {
            counter.count(entry);
        }
        // println!("{:?}: {}", name, counter);
        println!("{}", color::info(&counter));
    }
}

#[test]
fn test_readable_size() {
    let mut c = Counter::new();
    c.size = 1023;
    assert_eq!(c.readable_size(), "1023B");
    c.size = 1434;
    assert_eq!(c.readable_size(), "1.4K");
    c.size = 15926;
    assert_eq!(c.readable_size(), "15.6K");
    c.size = 53589793;
    assert_eq!(c.readable_size(), "51.1M");
    c.size = 238462643383;
    assert_eq!(c.readable_size(), "222.1G");
    c.size = 279502884197169;
    assert_eq!(c.readable_size(), "254.2T");
    c.size = 0xffffffffffffffff;
    assert_eq!(c.readable_size(), "16E");
}
