/// Improvementes
/// no need to implement the tree using hashmap, a vec should be enough
use std::collections::HashMap;

use crate::helpers::read;

type Filesystem = HashMap<String, Item>;

pub fn run() {
    let input = read::file_to_lines("day07").unwrap().flatten();
    let filesystem = process_input(input);

    println!("Day 07");
    println!(
        "Part 01, Sum of total sizes of directories at most 100.000b {}",
        sum_at_most(&filesystem)
    );
    println!(
        "Part 02, Size of the Dir to be deleted {}",
        find_big_enough_dir(&filesystem)
    );
}

fn sum_at_most(filesystem: &Filesystem) -> u32 {
    filesystem
        .iter()
        .filter_map(|(_, v)| {
            if v.item_type != ItemType::File && v.size <= 100_000 {
                Some(v.size)
            } else {
                None
            }
        })
        .sum()
}

fn find_big_enough_dir(filesystem: &Filesystem) -> u32 {
    const TOTAL_DISK_SPACE: u32 = 70_000_000;
    const UPDATE_SIZE: u32 = 30_000_000;

    let used_space = filesystem.get("/").unwrap().size;
    let free_space = TOTAL_DISK_SPACE - used_space;
    let needed_space = UPDATE_SIZE - free_space;

    filesystem
        .iter()
        .filter_map(|(_, v)| {
            if v.item_type == ItemType::Dir && v.size >= needed_space {
                Some(v.size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn process_input(input: impl Iterator<Item = String>) -> Filesystem {
    let mut filesystem: Filesystem = HashMap::new();
    let mut branch: Vec<String> = Vec::new();

    for line in input {
        // println!("{}", line);
        let mut chuncks = line.split_ascii_whitespace().take(3);
        let command = (chuncks.next(), chuncks.next(), chuncks.next());

        // let mut curr_dir: &Item;

        match command {
            (Some("$"), Some("ls"), None) => (), // do nothing

            (Some("$"), Some("cd"), Some(name @ "/")) => {
                let root = filesystem.entry(name.to_string()).or_insert(Item::new_root());
                branch.clear();
                branch.push(root.name.clone());
                // println!("{:?}", branch);
            }

            (Some("$"), Some("cd"), Some("..")) => {
                if branch.last().unwrap() != "/" {
                    branch.pop();
                }
                // println!("{:?}", branch);
            }

            (Some("$"), Some("cd"), Some(name)) => {
                // compounding names because there are directories with the same name
                let mut name = name.to_string();
                name.push_str(branch.last().unwrap());

                let dir = filesystem.entry(name.clone()).or_insert(Item::new_dir(&name));
                branch.push(dir.name.clone());
                // println!("{:?}", branch);
            }

            (Some("dir"), Some(_name), None) => (), // do nothing

            // insert file and updates sizes of the branch
            (Some(size), Some(name), None) => {
                // println!("{} {:?}", idx, branch);
                let size = str::parse::<u32>(size).unwrap();
                filesystem.insert(name.to_string(), Item::new_file(name, size));

                branch
                    .iter()
                    .for_each(|name| filesystem.get_mut(name).unwrap().size += size);
            }
            _ => panic!("Unexpect value in file."),
        };
    }
    filesystem
}

#[derive(Debug, PartialEq)]
enum ItemType {
    Dir,
    File,
    Root,
}

#[derive(Debug)]
struct Item {
    item_type: ItemType,
    name: String,
    size: u32,
}

impl Item {
    fn new_root() -> Self {
        Self {
            item_type: ItemType::Root,
            name: "/".to_owned(),
            size: 0,
        }
    }

    fn new_dir(name: &str) -> Self {
        Self {
            item_type: ItemType::Dir,
            name: name.to_owned(),
            size: 0,
        }
    }

    fn new_file(name: &str, size: u32) -> Self {
        Self {
            item_type: ItemType::File,
            name: name.to_owned(),
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: String = "$ cd /
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
7214296 k"
            .to_string();
    }

    #[test]
    fn teste_part_01() {
        let filesystem = process_input(INPUT.lines().map(|l| l.to_string()));
        assert_eq!(sum_at_most(&filesystem), 95437);
    }
}
