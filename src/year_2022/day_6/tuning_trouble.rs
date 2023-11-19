use itertools::Itertools;

fn first_unique_window_index(source: &String, window_size: usize) -> Option<usize> {
    source
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .position(|x| x.into_iter().all_unique())
}

fn message_marker_index(stream: &String) -> Option<usize> {
    let stream = &stream[5..].to_owned();
    first_unique_window_index(stream, 14)
}

fn packet_marker_index(stream: &String) -> Option<usize> {
    first_unique_window_index(stream, 4)
}

pub fn solve(lines: Vec<String>) {
    for line in lines {
        let start_mark_i = packet_marker_index(&line);
        match start_mark_i {
            Some(i) => {
                println!("packet marker starts after character {}", i + 4);
                let start_mark_i = message_marker_index(&line);
                match start_mark_i {
                    Some(i) => println!("message marker starts after character {}", i + 19),
                    None => println!("Could not find a marker"),
                }
            }
            None => println!("Could not find a marker"),
        };
    }
}
