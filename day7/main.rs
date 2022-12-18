
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
use indextree::{Arena, NodeId, Node};

fn main() {
    const INPUT_FILE: &str = "day7/input.txt";

    let input_str = std::fs::read_to_string(INPUT_FILE).unwrap();

    let lines : Vec<&str> = input_str.lines().collect();
    let fs = FileSystem::from(lines);

    println!("{}", fs);

    let size: usize =  fs.get_dir_sizes().iter().filter(|x| **x < 100000).sum();

    println!("a: {}", size);

    const NEEDED_SPACE: usize = 30000000;
    const OVERALL_SPACE: usize = 70000000;
    let overall_size: usize = *fs.get_dir_sizes().iter().max().unwrap();
    let available_space = OVERALL_SPACE-overall_size;
    let to_be_deleted_space = NEEDED_SPACE-available_space;
    println!("{} of {} available, need to remove {}", available_space, NEEDED_SPACE, to_be_deleted_space);

    let res_b: usize = *fs.get_dir_sizes().iter().filter(|x| **x>to_be_deleted_space).min().unwrap();

    println!("b: {}, dist={}", res_b, res_b - to_be_deleted_space);
}

#[derive(Debug)]
enum NodeData {
    File {name: String, size: usize },
    Directory{ name: String }
}

struct FileSystem {
    arena: Arena<NodeData>,
    root: NodeId,
}

impl FileSystem {
    
    fn get_subfolder(&self, folder: NodeId, name: &str) -> Option<NodeId> {

        if let Some(dir_node) = folder.children(&self.arena)
                .filter_map(|node_id| self.arena.get(node_id))
                .filter(|node| FileSystem::is_named(*node, name))
                .next()
        {
            return self.arena.get_node_id(dir_node)
        }

        None        
    }

    fn get_parentfolder(&self, folder: NodeId) -> Option<NodeId> {
        folder.ancestors(&self.arena)
            .skip(1)
            .next()
    }

    fn is_named(node: &Node<NodeData>, aname: &str) -> bool {

        let data = node.get();

        return match data {
            NodeData::Directory {name} => name == aname,
            _ => false,
        };
    }
    
    fn display_node(&self, f: &mut fmt::Formatter, node_id: NodeId, depth: usize) -> fmt::Result {
        let node = self.arena.get(node_id).ok_or(fmt::Error)?;

        let data = node.get();

        match data {
            NodeData::File {name, size} => {
                write!(f, "{}{} ({})\n", "\t".repeat(depth), name, size)?;
            },
            NodeData::Directory {name} => {
                write!(f, "{}{} (dir)\n", "\t".repeat(depth), name)?;
                for child in node_id.children(&self.arena) {
                    self.display_node(f, child, depth+1)?;
                }
            }
        };

        Ok(())
    }

    fn get_dir_sizes(&self) -> Vec<usize> {
        let mut result = Vec::new();

        self.get_dir_size(&mut result, self.root);

        result
    }

    fn get_dir_size(&self, result: &mut Vec<usize>, node_id: NodeId) -> usize {
        if let Some(node) = self.arena.get(node_id) {

            match node.get() {
                NodeData::File {size, ..} => {
                    return *size;
                },
                NodeData::Directory{..} => {
                    let mut dir_size: usize = 0;

                    for child in node_id.children(&self.arena) {
                        dir_size += self.get_dir_size(result, child);
                    }

                    result.push(dir_size);

                    return dir_size;
                }
            }
        }

        0

    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/\n")?;
        
        for node_id in self.root.children(&self.arena) { 
            self.display_node(f, node_id, 1)?;
        }

        Ok(())
    }
}

impl From<Vec<&str>> for FileSystem {

    fn from(lines: Vec<&str>) -> Self {
        let mut arena = Arena::new();

        let name = "root".to_string();
        let root = arena.new_node(NodeData::Directory{name});

        let mut fs = FileSystem {
            arena,
            root,        
        };
        
        let mut current_directory = fs.root;

        for line in lines {
            
            match Input::from_str(line) {
                Ok(Input::DirectoryEntry{name}) => {
                    let new_directory = fs.arena.new_node(
                        NodeData::Directory{name}
                    );
                    current_directory.append(new_directory, &mut fs.arena);
                },
                Ok(Input::FileEntry{name, size}) => {
                    let new_directory = fs.arena.new_node(
                        NodeData::File{name, size}
                    );
                    current_directory.append(new_directory, &mut fs.arena);
                },
                Ok(Input::ChangeDirCmd{target}) => {
                    current_directory = match &target[..] {
                        "/" => fs.root,
                        ".." => fs.get_parentfolder(current_directory).unwrap(),
                        _ => fs.get_subfolder(current_directory, &target).unwrap(),
                    }
                }
                _ => {},
            }
        }

        fs
    }
}



enum Input {
    ChangeDirCmd{target: String},
    ListCmd,
    DirectoryEntry{name: String},
    FileEntry{name: String, size: usize}
}

#[derive(Debug)]
struct ParseInputError;

impl From<ParseIntError> for ParseInputError {
    fn from(_err: ParseIntError) -> Self {
        ParseInputError{}
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd") {

            let target = s.split(" ")
                    .last()
                    .ok_or(ParseInputError{})?;
            
            Ok(Input::ChangeDirCmd{target: target.into()})

        } else if s.starts_with("$ ls") {

            Ok(Input::ListCmd)

        } else if s.starts_with("dir") {
            let name = s.split(" ")
                    .last()
                    .ok_or(ParseInputError{})?;
            
            Ok(Input::DirectoryEntry{name: name.to_string()})

        } else {

            let mut split_iter = s.split(" ");

            let size_str: &str = split_iter.next()
                                .ok_or(ParseInputError{})?;

            let size: usize = size_str.parse()?;

            let name = split_iter.next()
                       .ok_or(ParseInputError{})?;
            

            Ok(Input::FileEntry{name: name.to_string(), size})

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_works() {
        let lines = vec![
            "$ cd /",
            "$ ls",
            "dir abc",
            "123 dd",
            "$ cd abc",
            "$ ls",
            "dir bb",
            "123123 as"
        ];

        let fs = FileSystem::from(lines);

        let root = fs.root;

        assert_eq!(root.children(&fs.arena).count(), 2);
        
        let subfolder = fs.get_subfolder(fs.root, "abc").unwrap();
        
        assert_eq!(subfolder.children(&fs.arena).count(), 2);
    }

    #[test]
    fn input_from_str_works() {
       
        match Input::from_str("$ cd a").unwrap() {
            Input::ChangeDirCmd{target} => assert_eq!(target, "a"),
            _ => panic!("nonono")
        }

        match Input::from_str("$ ls").unwrap() {
            Input::ListCmd => {},
            _ => panic!()
        }

        match Input::from_str("dir asfas").unwrap() {
            Input::DirectoryEntry{name} => assert_eq!(name, "asfas"),
            _ => panic!(),
        }

        match Input::from_str("3243 sfasf.dat").unwrap() {
            Input::FileEntry{name, size} => {
                assert_eq!(name, "sfasf.dat");
                assert_eq!(size, 3243);
            },
            _ => panic!(),
        }
    }

}
