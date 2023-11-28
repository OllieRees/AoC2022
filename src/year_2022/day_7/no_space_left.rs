
#[derive(Debug, PartialEq, Eq, Clone)]
struct File {
    name: String,
    file_size: u64,
    parent: Option<Box<File>>,
    children: Option<Vec<File>>
}

pub fn solve(lines: Vec<String>) {

}

#[cfg(test)]
mod no_space_left {
    use crate::year_2022::day_7::no_space_left::File;

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
    fn test_cd_parent() {
        let input = "cd ..";
        let current_dir = File{ name: "a".to_owned(), file_size: 0, parent: Box::from(parent_dir), children: Some(vec![])};
        let parent_dir = File{name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![current_dir])};
        assert_eq!(execute_cd(Some(current_dir), input), parent_dir);
    }

    #[test]
    fn test_ls() {
        let current_dir: File = File{ name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![]) };
        let input: Vec<String> = vec!["ls", "dir a", "14848514 b.txt", "8504156 c.dat", "dir d"].into_iter().map(|s| s.to_owned()).collect();
        let expected: File = File{ name: "/".to_owned(), file_size: 0, parent: None, children: Some(vec![
            File { name: "a".to_owned(), file_size: 0, parent: Some(Box::from(current_dir)), children: Some(vec![]) },
            File { name: "b.txt".to_owned(), file_size: 14848514, parent: Some(Box::from(current_dir)), children: None },
            File { name: "c.dat".to_owned(), file_size: 8504156, parent: Some(Box::from(current_dir)), children: None },
            File { name: "d".to_owned(), file_size: 0, parent: Some(Box::from(current_dir)), children: Some(vec![]) },
        ]) };
        assert_eq!(execute_ls(current_dir, input), expected);
    }
}