use std::collections::HashMap;

#[derive(Debug)]
struct FileSystem {
    paths: Vec<Path>,
}

#[derive(Debug)]
struct Path {
    size: usize,
    parent: usize,
    children: HashMap<String, usize>,
}

pub fn solve() {
    let fs = parse(&std::fs::read_to_string("data/day07.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&fs));

    // part 2
    println!("part 2: {}", part2(&fs));
}

fn parse(data: &str) -> FileSystem {
    let mut fs = FileSystem {
        paths: vec![Path {
            size: 0,
            parent: 0,
            children: HashMap::new(),
        }],
    };
    let mut cd = 0;
    let mut ls = false;
    for line in data.lines() {
        if line == "$ ls" {
            ls = true;
        } else if line.starts_with("$ cd ") {
            ls = false;
            let (_, name) = line.split_at(5);
            cd = match name {
                "/" => 0,
                ".." => fs.paths[cd].parent,
                _ => *fs.paths[cd].children.get(name).expect("existing-directory"),
            }
        } else if ls {
            let (size, name) = line.split_once(' ').expect("ls-output-space");
            // ignore any paths we have already ls'ed
            if !fs.paths[cd].children.contains_key(name) {
                // insert the new node
                let child = fs.paths.len();
                fs.paths.push(Path {
                    size: if size != "dir" {
                        size.parse().expect("file-size")
                    } else {
                        0
                    },
                    parent: cd,
                    children: HashMap::new(),
                });
                fs.paths[cd].children.insert(String::from(name), child);
            }
        } else {
            panic!("unknown-line");
        }
    }
    fs
}

fn part1(fs: &FileSystem) -> usize {
    let mut cache = vec![0; fs.paths.len()];
    (0..fs.paths.len())
        .filter(|&id| fs.paths[id].size == 0)
        .map(|id| tally(fs, &mut cache, id))
        .filter(|&s| s < 100000)
        .sum()
}

fn part2(fs: &FileSystem) -> usize {
    let mut cache = vec![0; fs.paths.len()];
    let total = tally(fs, &mut cache, 0);
    (0..fs.paths.len())
        .filter(|&id| fs.paths[id].size == 0)
        .map(|id| tally(fs, &mut cache, id))
        .filter(|&s| 70000000 - (total - s) >= 30000000)
        .min()
        .expect("large-enough-directory")
}

fn tally(fs: &FileSystem, cache: &mut Vec<usize>, id: usize) -> usize {
    let child = &fs.paths[id];
    if child.size != 0 {
        child.size
    } else {
        if cache[id] == 0 {
            cache[id] = child
                .children
                .values()
                .map(|&id| tally(fs, cache, id))
                .sum();
        }
        cache[id]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    lazy_static! {
        static ref FS: FileSystem = parse(DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&FS), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&FS), 24933642);
    }
}
