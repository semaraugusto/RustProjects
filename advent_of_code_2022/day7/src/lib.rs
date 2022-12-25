use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, ops::Add, rc::Rc};

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    childs: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            name: "/".to_string(),
            size: RefCell::new(0),
            parent: None,
            childs: RefCell::new(HashMap::new()),
        }
    }
}
impl Dir {
    fn directory_size(&self) -> usize {
        let mut total = self.size.borrow().clone();
        for dir in self.childs.borrow().values() {
            total += dir.directory_size();
        }
        total
    }
}
#[derive(Default)]
struct Root {
    root: Rc<Dir>,
}

impl Root {
    fn parse_input(&mut self, input: &str) {
        let mut curr_dir = Rc::clone(&self.root);
        input.lines().for_each(|line| {
            println!("line: {}", line);
            match line {
                cmd if cmd.starts_with('$') => {
                    let cmd = cmd.trim_start_matches("$ ");
                    match cmd {
                        ls if ls.starts_with("ls") => (),
                        cd_cmd if cd_cmd.starts_with("cd") => {
                            let dir = cd_cmd.trim_start_matches("cd ");
                            match dir {
                                "/" => curr_dir = Rc::clone(&self.root),
                                ".." => curr_dir = Rc::clone(curr_dir.parent.as_ref().unwrap()),
                                dirname => {
                                    let newdir =
                                        curr_dir.childs.borrow().get(dirname).unwrap().clone();
                                    curr_dir = newdir;
                                }
                            }
                        }
                        _ => panic!("Invalid command"),
                    }
                }
                output => match output.split_once(' ').unwrap() {
                    (size, _name) if size.parse::<usize>().is_ok() => {
                        *curr_dir.size.borrow_mut() += size.parse::<usize>().unwrap();
                    }
                    ("dir", dirname) => {
                        curr_dir.childs.borrow_mut().insert(
                            dirname.to_string(),
                            Rc::new(Dir {
                                name: dirname.to_string(),
                                size: RefCell::new(0),
                                parent: Some(Rc::clone(&curr_dir)),
                                childs: RefCell::new(HashMap::new()),
                            }),
                        );
                    }
                    _ => panic!("Invalid output"),
                },
            }
        });
    }
}

fn part1(input: &str) -> u32 {
    let mut root = Root::default();
    let mut total = 0;
    root.parse_input(input);
    let mut to_visit = vec![Rc::clone(&root.root)];
    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.childs.borrow().values().cloned());
        let size = dir.directory_size();
        if size <= 100_000 {
            total += size as u32;
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let mut root = Root::default();
    root.parse_input(input);
    let free = 70_000_000 - root.root.directory_size();
    let mut to_visit = vec![Rc::clone(&root.root)];
    let min_needed = 30_000_000;
    let mut results = vec![];
    let mut all_sizes = vec![];
    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.childs.borrow().values().cloned());
        let size = dir.directory_size();

        if free + size >= min_needed {
            results.push(size);
        }
        all_sizes.push(size);
    }
    results.sort();
    all_sizes.sort();
    results[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_1() {
        let input = include_str!("../toy.txt");
        let result = super::part1(input);
        assert_eq!(result, 95437);
    }
    #[test]
    fn part1_2() {
        let input = include_str!("../input.txt");
        let result = super::part1(input);
        assert_eq!(result, 1348005);
    }
    #[test]
    fn part2_1() {
        let input = include_str!("../toy.txt");
        let result = super::part2(input);
        assert_eq!(result, 24933642);
    }
    #[test]
    fn part2_2() {
        let input = include_str!("../input.txt");
        let result = super::part2(input);
        assert_eq!(result, 12785886);
    }
}
