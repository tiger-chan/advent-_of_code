use std::fs;

fn main() {
    let data_str = fs::read_to_string("./data/height_map.txt").unwrap();

    let map_split = data_str.split("\n");
    let w = if let Some(n) = map_split.clone().nth(0) {
        n.len()
    } else {
        panic!("Malformed input data");
    };
    let h = map_split.clone().count();

    let mut height_map: Vec<u32> = Vec::new();
    height_map.reserve_exact(w * h);

    map_split.for_each(|f| {
        f.chars().for_each(|c| match c {
            '0'..='9' => {
                height_map.push(c as u32 - '0' as u32);
            }
            _ => {}
        })
    });

    let to_idx = |x: usize, y: usize| -> usize { x as usize + (y as usize * w) };

    enum ExtentResult {
        Blocked((usize, usize)),
        Max,
    }

    let extent_rev = |height, min_x, max_x, min_y, max_y| -> ExtentResult {
        for y in (min_y..max_y).rev() {
            for x in (min_x..max_x).rev() {
                let idx = to_idx(x, y);
                if let Some(n) = height_map.get(idx as usize) {
                    if n >= height {
                        return ExtentResult::Blocked((x, y));
                    }
                }
            }
        }
        return ExtentResult::Max;
    };

    let extent = |height, min_x, max_x, min_y, max_y| -> ExtentResult {
        for y in min_y..max_y {
            for x in min_x..max_x {
                let idx = to_idx(x, y);
                if let Some(n) = height_map.get(idx as usize) {
                    if n >= height {
                        return ExtentResult::Blocked((x, y));
                    }
                }
            }
        }
        return ExtentResult::Max;
    };

    let scenic_score = |o: (usize, usize), e: ExtentResult, m: (usize, usize)| -> usize {
        let end = match e {
            ExtentResult::Max => m,
            ExtentResult::Blocked(b) => b,
        };
        i32::abs(o.0 as i32 - end.0 as i32) as usize + i32::abs(o.1 as i32 - end.1 as i32) as usize
    };

    let mut max_score = 0;
    for y in 1..h - 1 {
        let h_y = y + 1;
        for x in 1..w - 1 {
            let h_x = x + 1;
            let idx = to_idx(x, y);
            let origin = (x, y);
            if let Some(c) = height_map.get(idx as usize) {
                let mut scores: [usize; 4] = [1; 4];

                scores[0] = scenic_score(origin, extent_rev(c, x, h_x, 0, y), (x, 0)); // up
                scores[1] = scenic_score(origin, extent_rev(c, 0, x, y, h_y), (0, y)); // left
                scores[2] = scenic_score(origin, extent(c, x, h_x, h_y, h), (x, h - 1)); // down
                scores[3] = scenic_score(origin, extent(c, h_x, w, y, h_y), (w - 1, y)); // right

                let score = scores.iter().fold(1, |x, i| x * i);

                if max_score < score {
                    max_score = score;
                }
            }
        }
    }

    println!("[2022][08] max scenic: {}", max_score);
}
