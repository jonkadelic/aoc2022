use core::slice;

pub const PUZZLE: Puzzle = Puzzle { name: "day7_1" };

pub struct Puzzle {
    name: &'static str
}

impl aoc_common::Puzzle for Puzzle {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_expected_test_result(&self) -> &str {
        "95437"
    }

    fn get_result(&self, lines: Vec<String>) -> String {
        let mut fs = Filesystem::new();
        let mut cwd: DirIndex = 0;
        let mut i: usize = 1;
        let mut slice_start: usize = 0;
        loop {
            if i == lines.len() || lines[i].starts_with("$") {
                match parse_command(&mut fs, cwd, &lines[slice_start..i]) {
                    Some(val) => cwd = val,
                    None => ()
                }
                slice_start = i;
                i = slice_start;
            }
            if i == lines.len() {
                break;
            } else {
                i += 1;
            }
        }

        let mut total_size: usize = 0;
        for dir in &fs.dirs {
            let dir_size = dir.get_size(&fs);
            if dir_size <= 100000 {
                total_size += dir_size;
            }
        }

        total_size.to_string()
    }
}

fn parse_command(fs: &mut Filesystem, cwd: DirIndex, lines: &[String]) -> Option<DirIndex> {
    let parts = lines[0].split_whitespace().collect::<Vec<&str>>();

    if parts[0] == "$" {
        if parts[1] == "cd" {
            if parts[2] == ".." {
                let parent_dir: DirIndex = match fs.get_parent_dir(cwd) {
                    Some(val) => val,
                    None => panic!("Tried to go up from root dir!")
                };
                return Some(parent_dir);
            } else if parts[2] == "/" {
                return Some(0);
            } else {
                let cwdir = &fs.dirs[cwd];
                for d in &cwdir.dirs {
                    let subdir = &fs.dirs[*d];
                    if subdir.get_name() == parts[2] {
                        return Some(*d);
                    }
                }
                panic!("Tried to CD to non-existent dir!");
            }
        } else if parts[1] == "ls" {
            for line in &lines[1..] {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                let fs_files_len = fs.files.len();
                let fs_dirs_len = fs.dirs.len();
                let cwdir = &mut fs.dirs[cwd];
                if parts[0] == "dir" {
                    let new_dir = Directory { name: parts[1].to_string(), files: vec![], dirs: vec![] };
                    cwdir.dirs.push(fs_dirs_len);
                    fs.dirs.push(new_dir);
                } else {
                    let size = parts[0].parse::<usize>().unwrap();
                    let new_file = File { name: parts[1].to_string(), size };
                    cwdir.files.push(fs_files_len);
                    fs.files.push(new_file);
                }
            }
        } else {
            panic!("Unknown command!");
        }
    } else {
        panic!("First line of input was not a command!");
    }

    None
}

type FileIndex = usize;
type DirIndex = usize;

struct Filesystem {
    files: Vec<File>,
    dirs: Vec<Directory>
}

impl Filesystem {
    fn new() -> Filesystem {
        let mut fs = Filesystem { files: vec![], dirs: vec![] };
        fs.dirs.push(Directory { name: String::new(), files: vec![], dirs: vec![] });

        fs
    }

    fn get_parent_dir(&self, dir: DirIndex) -> Option<DirIndex> {
        for d in self.dirs.iter().enumerate() {
            if d.1.dirs.contains(&dir) {
                return Some(d.0);
            }
        }
        None
    }
}

trait FilesystemMember {
    fn get_name(&self) -> &String;

    fn get_size<'a>(&self, fs: &Filesystem) -> usize;
}

struct File {
    name: String,
    size: usize
}

impl FilesystemMember for File {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_size<'a>(&self, _: &Filesystem) -> usize {
        return self.size;
    }
}

struct Directory {
    name: String,
    files: Vec<FileIndex>,
    dirs: Vec<DirIndex>
}

impl FilesystemMember for Directory {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn get_size(&self, fs: &Filesystem) -> usize {
        let mut size = 0;
        for file_index in &self.files {
            let f = fs.files[*file_index].get_size(fs);
            size += f;
        }
        for dir_index in &self.dirs {
            let d = fs.dirs[*dir_index].get_size(fs);
            size += d;
        }
        size
    }
}