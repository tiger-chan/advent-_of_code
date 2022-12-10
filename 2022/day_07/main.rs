use std::fs;

const TOTAL_FS: u32 = 70000000;
const SPACE_NEEDED: u32 = 30000000;

fn main() {
    let data_str = fs::read_to_string("./data/os.txt").unwrap();

    let mut iter = data_str.split("\n").peekable();
    let mut folders: Vec<(String, u32)> = Vec::new();
    let mut stack: Vec<(String, u32)> = Vec::new();
    while let Some(line) = iter.next() {
        match line {
            line if line.starts_with("$ cd") => {
                let (_cmd, dir) = line.split_at("$ cd ".len());
                match dir {
                    _ if dir.starts_with('/') => {
                        stack.push(("/".to_owned(), 0));
                    }
                    _ if dir.starts_with("..") => {
                        if stack.len() == 0 {
                            break;
                        }
                        if let Some(top) = stack.pop() {
                            if let Some(new_top) = stack.last_mut() {
                                new_top.1 += top.1;
                            }

                            folders.push(top);
                        }
                    }
                    dir => {
                        stack.push((dir.to_owned(), 0));
                    }
                }
            }
            line if line.starts_with("$ ls") => {
                while let Some(output) = iter.peek() {
                    match output {
                        output if output.starts_with("$") => {
                            break;
                            // we've gone until we reached another command
                        }
                        output if output.starts_with("dir") => {
                            // <type> name
                        }
                        output => {
                            // <size> name
                            if let Some(num_str) = output.split_whitespace().nth(0) {
                                if let Ok(num) = u32::from_str_radix(num_str, 10) {
                                    if let Some(new_top) = stack.last_mut() {
                                        new_top.1 += num;
                                    }
                                }
                            }
                        }
                    }
                    iter.next();
                }
            }
            _ => {}
        }
    }

    let remaining_stack = stack.len();
    for _ in 0..remaining_stack {
        if let Some(top) = stack.pop() {
            if let Some(new_top) = stack.last_mut() {
                new_top.1 += top.1;
            }

            folders.push(top);
        }
    }

    folders.sort_by(|a, b| a.1.cmp(&b.1));

    let total = if let Some(f) = folders.last() { f.1 } else { 0 };

    let remaining: u32 = TOTAL_FS - total;
    let mut deleted = 0;
    for folder in &folders {
        if remaining + folder.1 > SPACE_NEEDED {
            deleted = folder.1;
            break;
        }
    }

    println!(
        "[2022][07] deleted: {} total: {} Folders: {:?}",
        deleted, total, folders
    );
}
