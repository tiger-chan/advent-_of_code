use std::fs;

const MAX_FOLDER_SIZE: u32 = 100000;

fn main() {
    let data_str = fs::read_to_string("./data/os.txt").unwrap();

    // Creating a full array of all enties in the FS
    // Keep track of each items size / contents

    // treat all items as files?
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

                            if top.1 < MAX_FOLDER_SIZE {
                                folders.push(top);
                            }
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

    let mut total = 0;
    for folder in &folders {
        total += folder.1;
    }

    println!("[2022][07] total: {} Folders: {:?}", total, folders);
}
