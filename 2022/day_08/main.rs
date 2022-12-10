use std::fs;

fn main() {
    let data_str = fs::read_to_string("./data/height_map.txt").unwrap();

    let map_split = data_str.split("\n");
    let width = if let Some(n) = map_split.clone().nth(0) {
        n.len()
    } else {
        panic!("Malformed input data");
    };
    let height = map_split.clone().count();

    let mut height_map: Vec<u32> = Vec::new();
    height_map.reserve_exact(width * height);

    map_split.for_each(|f| {
        f.chars().for_each(|c| match c {
            '0'..='9' => {
                height_map.push(c as u32 - '0' as u32);
            }
            _ => {}
        })
    });

    let mut visible_count = 2 * height + 2 * width - 4 /* Don't double count the corners */;

    let to_idx = |x: usize, y: usize| -> usize { x as usize + (y as usize * width) };
    let is_visible = |height, min_x, min_y, max_x, max_y| -> bool {
        for y in min_y..max_y {
            for x in min_x..max_x {
                let idx = to_idx(x, y);
                if let Some(n) = height_map.get(idx as usize) {
                    if n >= height {
                        return false;
                    }
                }
            }
        }
        return true;
    };

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = to_idx(x, y);
            if let Some(h) = height_map.get(idx as usize) {
                if is_visible(h, 0, y, x, y + 1)
                    || is_visible(h, x + 1, y, width, y + 1)
                    || is_visible(h, x, 0, x + 1, y)
                    || is_visible(h, x, y + 1, x + 1, height)
                {
                    visible_count += 1;
                }
            }
        }
    }

    println!("[2022][08] visible: {}", visible_count);
}
