// Very ugly solution - need to look at how I can improve the use of mutability in the
// execute_command
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

type Stack = VecDeque<char>;
type Stacks = Vec<RefCell<Stack>>;

fn parse_command(command_line: String) -> Option<(usize, usize, usize)> {
    //regex follows pattern of "move %d from %d to %d"
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    if CMD_RE.is_match(command_line.as_str()) {
        if let Some(capture_arr) = CMD_RE.captures(command_line.as_str()) {
            return Some((
                capture_arr[1].parse::<usize>().unwrap(),
                capture_arr[2].parse::<usize>().unwrap(),
                capture_arr[3].parse::<usize>().unwrap(),
            ));
        }
    }
    None
}

fn get_last_empty_box(iterator: Iter<char>) -> Option<usize> {
    match iterator
        .enumerate()
        .filter(|(_, c)| c.to_owned().to_owned() == ' ')
        .last()
    {
        Some((i, _)) => Some(i),
        None => None,
    }
}

// TODO: Combine the operations on src
// TODO: Have src and dest borrow_mut from the get-go
fn execute_command(stack: &mut Stacks, count: usize, src_i: usize, dest_i: usize, use_9001: bool) {
    // new stack is the same but with the first x of stack[src_i] are the first x of stack[dest_i]
    match (stack.get(src_i), stack.get(dest_i)) {
        (Some(src), Some(dest)) => {
            let top_box_start = src.borrow().iter().position(|c| *c != ' ').unwrap_or(0);
            let count = [count, src.borrow().len()]
                .into_iter()
                .min()
                .unwrap_or(count);

            let mut src_mut = src.borrow_mut();

            let mut delta_boxes = src_mut
                .drain(top_box_start..top_box_start + count)
                .collect::<Vec<char>>();

            // reverse for part 2
            if use_9001 {
                delta_boxes.reverse();
            }

            // concatenate delta-boxes onto dest
            for d_box in delta_boxes {
                let stack_top_index = get_last_empty_box(dest.borrow().iter());
                match stack_top_index {
                    Some(i) => {
                        std::mem::replace(&mut dest.borrow_mut()[i], d_box);
                    }
                    None => dest.borrow_mut().push_front(d_box),
                }
                // replace postion d_box was in with an empty box
                src_mut.push_front(' ');
            }
        }
        _ => (),
    }
}

fn parse_stacks_chunks(stack_details: Vec<String>) -> Stacks {
    let stack_grid: Vec<Vec<char>> = stack_details
        .into_iter()
        .map(|row| {
            row.chars()
                .enumerate()
                .filter(|(i, _)| i % 4 == 1)
                .map(|(_, c)| c)
                .collect()
        })
        .collect();

    // all chars that share index are put in a vector which is indexed by that shared index
    let mut stack: Stacks = Vec::new();
    for row in stack_grid {
        for (i, c) in row.into_iter().enumerate() {
            if let Some(v) = stack.get(i) {
                v.borrow_mut().push_back(c);
            } else {
                stack.insert(i, RefCell::from(VecDeque::from([c])));
            }
        }
    }
    stack
}

fn get_init_stack_size(input_lines: &Vec<String>) -> (usize, String) {
    input_lines
        .clone()
        .into_iter()
        .find_position(|line| {
            line.split_whitespace()
                .all(|x| x.parse::<u32>().ok().is_some())
        })
        .unwrap()
}

fn stack_top(stacks: Stacks) -> Vec<char> {
    // go through each stack and get the first box that isn't ' '
    let mut top_stack = Vec::new();
    for stack in stacks {
        let stack = stack.borrow();
        match stack.iter().find(|c| **c != ' ') {
            Some(top_box) => top_stack.push(*top_box),
            None => (),
        }
    }
    top_stack
}

fn solution(lines: &Vec<String>, use_90001: bool) {
    let (last_stack_index, _) = get_init_stack_size(&lines);
    let stack_lines = lines[..last_stack_index].to_vec();
    let stacks: &mut Stacks = &mut parse_stacks_chunks(stack_lines);

    for command in lines[last_stack_index + 2..].to_vec() {
        if let Some((cnt, src_stack, dest_stack)) = parse_command(command) {
            if src_stack > 0 && dest_stack > 0 {
                execute_command(stacks, cnt, src_stack - 1, dest_stack - 1, use_90001);
            }
        }
    }

    println!("{:?}", stack_top(stacks.to_owned()));
}

pub fn main(lines: Vec<String>) {
    solution(&lines, false);
    solution(&lines, true);
}
