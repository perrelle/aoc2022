use std::collections::HashMap;

pub struct Dir {
    files: HashMap<String, u32>,
    subdirs: HashMap<String, Dir>
}

mod parser  {
    use nom::{
        IResult,
        character::complete::*, bytes::complete::tag,
        combinator::*, sequence::*, branch::*, multi::*
    };
        
    #[derive(Clone)]
    pub enum Path {
        Root,
        Parent,
        Subdir(String)
    }

    #[derive(Clone)]
    pub enum DirEntry {
        Dir(String),
        File(String, u32)
    }

    #[derive(Clone)]
    pub enum Command {
        ChangeDirectory(Path),
        ListDirectory(Vec<DirEntry>)
    }

    fn name(input: &str) -> IResult<&str, String> {
        fold_many1(
            satisfy(|c| c.is_alphanumeric() || c == '.'),
            String::new,
            |mut s, c| { s.push(c); s }
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<Command>> {
        let root = value(Path::Root, tag("/"));
        let parent = value(Path::Parent, tag(".."));
        let subdir = map(name, Path::Subdir);
        let path = alt((root, parent, subdir));
        let dir = map(
            preceded(terminated(tag("dir"), space1), name),
            DirEntry::Dir);
        let file = map(
            separated_pair(u32, space1, name),
            |(size,name)| DirEntry::File(name,size));
        let entry = alt((dir, file));
        let cd = preceded(
            terminated(tag("$ cd"), multispace1),
            map(path, Command::ChangeDirectory));
        let ls = preceded(
            terminated(tag("$ ls"), multispace1),
            map(separated_list0(multispace1, entry), Command::ListDirectory));
        let commands = separated_list1(multispace1, alt((cd, ls)));
        all_consuming(terminated(commands, multispace0))(input)
    }
}

fn access_path<'a>(root: &'a mut Dir, path: &Vec<String>) -> &'a mut Dir {
    let mut d = root;
    for name in path {
        d = d.subdirs.get_mut(name).unwrap();
    }
    d
}

pub fn process_command(
        root: &mut Dir,
        current: &mut Vec<String>,
        command: parser::Command) {
    match command {
        parser::Command::ChangeDirectory(path)  =>
            match path {
                    parser::Path::Root => { current.clear() },
                    parser::Path::Parent => { current.pop(); },
                    parser::Path::Subdir(name) => { current.push(name) }
            },
        parser::Command::ListDirectory(entries) => {
            let dir = access_path(root, current);
            assert!(dir.files.is_empty() && dir.subdirs.is_empty());
            for entry in entries {
                match entry {
                    parser::DirEntry::Dir(name) => {
                        let new_dir = Dir {
                            files: HashMap::new(),
                            subdirs: HashMap::new()
                        };
                        let _ = dir.subdirs.insert(name.clone(), new_dir);
                    },
                    parser::DirEntry::File(name, size) => {
                        let _ = dir.files.insert(name.clone(), size);
                    }
                }
            }
        }
    }
}

pub fn print_tree(root: &Dir) {
    fn print(current: &Dir, indent: usize) {
        for (name,subdir) in current.subdirs.iter() {
            println!("{}- {} (dir)", " ".repeat(indent), name);
            print(subdir, indent + 2);
        }
        for (name,size) in current.files.iter() {
            println!("{}- {}: {}", " ".repeat(indent), name, size);
        }
    }
    print(root, 0)
}

pub fn solve(input: &str) -> Option<(u32,u32)> {
    let (_,data) = parser::parse(input).unwrap();

    let mut root = Dir {
        files: HashMap::new(),
        subdirs: HashMap::new() 
    };
    let mut current = Vec::new();

    for command in data {
        process_command(&mut root, &mut current, command);
    }

    fn aux1(current: &Dir) -> (u32,u32) {
        let mut total_size = 0;
        let mut solution1 = 0;
        for (_name,subdir) in current.subdirs.iter() {
            let (sub_size,sub_solution1) = aux1(subdir);
            total_size += sub_size;
            solution1 += sub_solution1;
        }
        for (_name,size) in current.files.iter() {
            total_size += size;
        }
        if total_size <= 100000 {
            solution1 += total_size;
        }
        (total_size, solution1)
    }

    fn aux2(current: &Dir, needed_space: u32) -> (u32,u32) {
        let mut total_size = 0;
        let mut solution2 = 30000000;
        for (_name,subdir) in current.subdirs.iter() {
            let (sub_size,sub_solution2) = aux2(subdir, needed_space);
            total_size += sub_size;
            if sub_solution2 >= needed_space && sub_solution2 < solution2 {
                solution2 = sub_solution2;
            }
        }
        for (_name,size) in current.files.iter() {
            total_size += size;
        }
        if total_size >= needed_space && total_size < solution2 {
            solution2 = total_size;
        }
        (total_size, solution2)
    }

    let (total_size,solution1) = aux1(&root);
    let needed_space = total_size - 40000000;
    let (_,solution2) = aux2(&root, needed_space);
    
    Some ((solution1,solution2))
}

#[test]
fn test7_1() {
    let solution = solve(&include_str!("../inputs/day7.1"));
    assert_eq!(solution, Some ((95437,24933642)));
}

#[test]
fn test7_2() {
    let solution = solve(&include_str!("../inputs/day7.2"));
    assert_eq!(solution, Some ((1428881,10475598)));
}
