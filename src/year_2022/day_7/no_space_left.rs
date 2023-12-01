
use petgraph::{graph::Graph, visit::Bfs, EdgeDirection};


// This basically just a (file)tree
#[derive(Debug, PartialEq, Eq, Clone)]

struct File {
    name: String,
    file_size: u64,
}

pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
mod no_space_left {
    use crate::year_2022::day_7::no_space_left::File;

    const EXAMPLE: &str = "$ cd /
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
    7214296 k";
    

    #[test] 
    fn test_cd() {
        let input = "cd /";
        assert_eq!(execute_cd(None, input), File{name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![])});
    }

    #[test]
    fn test_cd_child() {
        let input = "cd a";
        let current_dir = File{name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![File { name: "a".to_owned(), file_size: 0, parent: None, children: Some(vec![]) }])};
        assert_eq!(execute_cd(Some(current_dir), input), File{name: "a".to_owned(), file_size: 0, parent: Some(Box::from(current_dir)), children: Some(vec![])});
    }

    #[test]
    fn test_cd_multiple_child() {
        let input = "cd a/e";
        let mut current_dir = File{name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![File { name: "a".to_owned(), file_size: 0, parent: None, children: None }])};
        let mut middle_dir = File { name: "a".to_owned(), file_size: 0, parent: Some(Box::from(current_dir)), children: None }; 
        current_dir.children = Some(vec![middle_dir]);
        let destination = File { name: "e".to_owned(), file_size: 0, parent: Some(Box::from(middle_dir)), children: Some(vec![]) };
        middle_dir.children = Some(vec![destination]);
        assert_eq!(execute_cd(Some(current_dir), input), File{name: "a".to_owned(), file_size: 0, parent: Some(Box::from(current_dir)), children: Some(vec![])});
    }

    #[test]
    fn test_cd_parent() {
        let input = "cd ..";
        let mut current_dir = File{ name: "a".to_owned(), file_size: 0, parent: None, children: Some(vec![])};
        let parent_dir = File{name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![current_dir])};
        current_dir.parent = Some(Box::from(parent_dir));
        assert_eq!(execute_cd(Some(current_dir), input), parent_dir);
    }

    #[test]
    fn test_ls() {
        let input: Vec<String> = vec!["ls".to_owned(), "dir a".to_owned(), "14848514 b.txt".to_owned(), "8504156 c.dat".to_owned(), "dir d".to_owned()];
        let current_dir: File = File{ name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![]) };
        let expected: File = File{ name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![
            File { name: "a".to_owned(), file_size: 0, parent: Some(Box::from(current_dir)), children: Some(vec![]) },
            File { name: "b.txt".to_owned(), file_size: 14848514, parent: Some(Box::from(current_dir)), children: None },
            File { name: "c.dat".to_owned(), file_size: 8504156, parent: Some(Box::from(current_dir)), children: None },
            File { name: "d".to_owned(), file_size: 0, parent: Some(Box::from(current_dir)), children: Some(vec![]) },
        ]) };
        assert_eq!(execute_ls(current_dir, input), expected);
    }
}